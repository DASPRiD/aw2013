# Rust AW2013 driver

This is a `std` driver for the AW2013 3-Channel LED Controller.

## RGB LED wiring

While the controller itself does not care how you wire an RGB LED to it, it is recommended to wire red, green and blue
to LED 0, 1 and 2 respectively.

## Examples

You can find examples in the `examples` directory which demonstrate the different use cases.

## Quirks of breathing mode

The datasheet of the AW2013 is not clear about setting a maximum brightness in this mode. With the brightness for each
LED set to either `0x00` or `0xff` there is no issue. When using values in between it affects the actual timing of the
controller. Lower values will thus result in the breathing cycle getting shorter than configured. This leads to the
following issues:

- With a brightness value too low and a relatively short breathing cycle an LED looks like it'd blink instead of
  breathing.
- With different brightness values for each LED but the same timing set they will quickly go out of sync. 

Thus, you should always follow these advices if not using `0x00` or `0xff` as a brightness value:

- Always use brightness values >= `0x32`.
- Always use the same brightness value for all active LEDs.
