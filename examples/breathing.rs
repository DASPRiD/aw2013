use aw2013::{Aw2013, Current, Led, Timing};

fn main() {
    let mut aw2013 = Aw2013::from_default_address([Current::Five; 3]).unwrap();
    aw2013.reset().unwrap();
    aw2013.enable().unwrap();
    aw2013
        .set_breathing(
            Led::Led0,
            255,
            &Timing {
                cycles: 0,
                delay: 0,
                rise: 2,
                hold: 2,
                fall: 2,
                off: 1,
            },
        )
        .unwrap();
}
