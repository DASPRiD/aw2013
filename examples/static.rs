use aw2013::{Aw2013, Current, Led};

fn main() {
    let mut aw2013 = Aw2013::from_default_address([Current::Five; 3]).unwrap();
    aw2013.reset().unwrap();
    aw2013.enable().unwrap();
    aw2013.set_static(Led::Led0, 128, None, None).unwrap();
}
