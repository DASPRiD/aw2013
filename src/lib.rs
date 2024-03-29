//! Driver for the AW2013 3-Channel LED Controller
//!
//! Provides a simple interface for the AW2013 LED controller.

#![no_std]

use embedded_hal::i2c::I2c;

// Register addresses
const REG_RESET: u8 = 0x00;
const REG_GLOBAL_CONTROL: u8 = 0x01;
const REG_LED_ENABLE: u8 = 0x30;
const REG_LED_MODE_BASE: u8 = 0x31;
const REG_LED_PWM_BASE: u8 = 0x34;
const REG_TIMING_0_BASE: u8 = 0x37;
const REG_TIMING_1_BASE: u8 = 0x38;
const REG_TIMING_2_BASE: u8 = 0x39;

// Register bits
const LED_MODULE_ENABLE_MASK: u8 = 0x01;
const LED_FADE_OUT_MASK: u8 = 0x40;
const LED_FADE_IN_MASK: u8 = 0x20;
const LED_BREATHE_MODE_MASK: u8 = 0x10;
const LED_RESET_MASK: u8 = 0x55;

/// LED mapping for the three different LEDs as defined by the specs.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Led {
    Led0 = 0x0,
    Led1 = 0x1,
    Led2 = 0x2,
}

/// Current to drive an LED in milliamps.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Current {
    Zero = 0x0,
    Five = 0x1,
    Ten = 0x2,
    Fifteen = 0x3,
}

/// Timing configuration for breathing effects.
///
/// If a supplied value is set too high, it is automatically clamped to the
/// allowed range.
pub struct Timing {
    /// Delay time before breathing cycle starts.
    ///
    /// - 0: 0s
    /// - 1: 0.13s
    /// - 2: 0.26s
    /// - 3: 0.52s
    /// - 4: 1.04s
    /// - 5: 2.08s
    /// - 6: 4.16s
    /// - 7: 8.32s
    /// - 8: 16.64s
    pub delay: u8,

    /// Rise period of breathing cycle.
    ///
    /// - 0: 0.13s
    /// - 1: 0.26s
    /// - 2: 0.52s
    /// - 3: 1.04s
    /// - 4: 2.08s
    /// - 5: 4.16s
    /// - 6: 8.32s
    /// - 7: 16.64s
    pub rise: u8,

    /// Hold period of the breathing cycle.
    ///
    /// - 0: 0.13s
    /// - 1: 0.26s
    /// - 2: 0.52s
    /// - 3: 1.04s
    /// - 4: 2.08s
    /// - 5: 4.16s
    pub hold: u8,

    /// Fall period of breathing cycle.
    ///
    /// - 0: 0.13s
    /// - 1: 0.26s
    /// - 2: 0.52s
    /// - 3: 1.04s
    /// - 4: 2.08s
    /// - 5: 4.16s
    /// - 6: 8.32s
    /// - 7: 16.64s
    pub fall: u8,

    /// Off period of breathing cycle.
    ///
    /// - 0: 0.13s
    /// - 1: 0.26s
    /// - 2: 0.52s
    /// - 3: 1.04s
    /// - 4: 2.08s
    /// - 5: 4.16s
    /// - 6: 8.32s
    /// - 7: 16.64s
    pub off: u8,

    /// Number of breathing cycles.
    ///
    /// Zero cycles means infinite, maximum configurable cycles is 15.
    pub cycles: u8,
}

pub struct Aw2013<I>
where
    I: I2c,
{
    i2c: I,
    address: u8,
    max_currents: [Current; 3],
}

impl<I> Aw2013<I>
where
    I: I2c,
{
    /// Create a new AW2013 driver from a pre-configured i2c interface.
    pub fn new(i2c: I, address: u8, max_currents: [Current; 3]) -> Self {
        Self {
            i2c,
            address,
            max_currents,
        }
    }

    /// Create a new AW2013 driver from the default address.
    pub fn from_default_address(i2c: I, max_currents: [Current; 3]) -> Self {
        Aw2013::new(i2c, 0x45, max_currents)
    }

    /// Reset the controller to its default state.
    ///
    /// Remember to enable the controller again after the reset if you plan to use it further.
    pub fn reset(&mut self) -> Result<(), I::Error> {
        self.write_register(REG_RESET, LED_RESET_MASK)
    }

    /// Enable the LED controller.
    pub fn enable(&mut self) -> Result<(), I::Error> {
        self.write_register(REG_GLOBAL_CONTROL, LED_MODULE_ENABLE_MASK)
    }

    /// Disable the LED controller.
    pub fn disable(&mut self) -> Result<(), I::Error> {
        self.write_register(REG_GLOBAL_CONTROL, 0)
    }

    /// Set a static RGB value for all LEDs.
    ///
    /// You can optionally define fade-in and fade-out effects to fade to or from other values.
    pub fn set_static_rgb(
        &mut self,
        rgb: [u8; 3],
        fade_in: Option<u8>,
        fade_out: Option<u8>,
    ) -> Result<(), I::Error> {
        for led in [Led::Led0, Led::Led1, Led::Led2] {
            self.set_static(led, rgb[led as usize], fade_in, fade_out)?;
        }

        Ok(())
    }

    /// Set a static value for a single LED.
    ///
    /// You can optionally define fade-in and fade-out effects to fade to or from other values.
    pub fn set_static(
        &mut self,
        led: Led,
        brightness: u8,
        fade_in: Option<u8>,
        fade_out: Option<u8>,
    ) -> Result<(), I::Error> {
        if brightness == 0 {
            return self.disable_led(led);
        }

        let mut config: u8 = self.max_currents[led as usize] as u8;

        if let Some(fade_in) = fade_in {
            config |= LED_FADE_IN_MASK;
            self.write_register(REG_TIMING_0_BASE + (led as u8) * 3, fade_in.min(7) << 4)?;
        }

        if let Some(fade_out) = fade_out {
            config |= LED_FADE_OUT_MASK;
            self.write_register(REG_TIMING_0_BASE + (led as u8) * 3, fade_out.min(7) << 4)?;
        }

        self.write_register(REG_LED_MODE_BASE + (led as u8), config)?;
        self.write_register(REG_LED_PWM_BASE + (led as u8), brightness)?;

        self.enable_led(led)?;

        Ok(())
    }

    /// Set a breathing cycle RGB value for all LEDs.
    pub fn set_breathing_rgb(&mut self, rgb: [u8; 3], timing: &Timing) -> Result<(), I::Error> {
        self.write_register(REG_LED_ENABLE, 0x0)?;

        for led in [Led::Led0, Led::Led1, Led::Led2] {
            self.write_register(
                REG_LED_MODE_BASE + (led as u8),
                self.max_currents[led as usize] as u8,
            )?;
        }

        for led in [Led::Led0, Led::Led1, Led::Led2] {
            self.write_register(REG_LED_PWM_BASE + (led as u8), rgb[led as usize])?;
            self.configure_timing(led, timing)?;
        }

        for led in [Led::Led0, Led::Led1, Led::Led2] {
            self.write_register(
                REG_LED_MODE_BASE + (led as u8),
                self.max_currents[led as usize] as u8 | LED_BREATHE_MODE_MASK,
            )?;
        }

        let mut active_leds = 0;

        for (i, value) in rgb.iter().enumerate() {
            if *value > 0 {
                active_leds |= 1 << i;
            }
        }

        self.write_register(REG_LED_ENABLE, active_leds)?;

        Ok(())
    }

    /// Set a breathing cycle value for a single LED.
    pub fn set_breathing(
        &mut self,
        led: Led,
        brightness: u8,
        timing: &Timing,
    ) -> Result<(), I::Error> {
        self.disable_led(led)?;

        if brightness == 0 {
            return Ok(());
        }

        self.write_register(REG_LED_PWM_BASE + (led as u8), brightness)?;
        self.configure_timing(led, timing)?;
        self.write_register(
            REG_LED_MODE_BASE + (led as u8),
            self.max_currents[led as usize] as u8 | LED_BREATHE_MODE_MASK,
        )?;

        self.enable_led(led)?;

        Ok(())
    }

    fn configure_timing(&mut self, led: Led, timing: &Timing) -> Result<(), I::Error> {
        self.write_register(
            REG_TIMING_0_BASE + (led as u8) * 3,
            timing.rise.min(7) << 4 | timing.hold.min(5),
        )?;
        self.write_register(
            REG_TIMING_1_BASE + (led as u8) * 3,
            timing.fall.min(7) << 4 | timing.off.min(7),
        )?;
        self.write_register(
            REG_TIMING_2_BASE + (led as u8) * 3,
            timing.delay.min(7) << 4 | timing.cycles.min(15),
        )?;

        Ok(())
    }

    fn disable_led(&mut self, led: Led) -> Result<(), I::Error> {
        let enable_value = self.read_register(REG_LED_ENABLE)?;
        self.write_register(REG_LED_ENABLE, enable_value & (!(1 << (led as u8))))
    }

    fn enable_led(&mut self, led: Led) -> Result<(), I::Error> {
        let enable_value = self.read_register(REG_LED_ENABLE)?;
        self.write_register(REG_LED_ENABLE, enable_value | (1 << (led as u8)))
    }

    fn write_register(&mut self, address: u8, data: u8) -> Result<(), I::Error> {
        self.i2c.write(self.address, &[address, data])
    }

    fn read_register(&mut self, address: u8) -> Result<u8, I::Error> {
        let mut buffer: [u8; 1] = [0];
        self.i2c.write_read(self.address, &[address], &mut buffer)?;
        Ok(buffer[0])
    }
}
