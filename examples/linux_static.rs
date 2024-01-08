use linux_embedded_hal::I2cdev;

use aw2013::{Aw2013, Current, Led};

fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap();
    let mut aw2013 = Aw2013::from_default_address(i2c, [Current::Five; 3]);
    aw2013.reset().unwrap();
    aw2013.enable().unwrap();
    aw2013.set_static(Led::Led0, 128, None, None).unwrap();
}
