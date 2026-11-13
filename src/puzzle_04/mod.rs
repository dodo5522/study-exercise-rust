use std::num::Wrapping;

pub fn overflow() {
    let mut counter = Wrapping(0_i8);
    loop {
        counter += 1;
        println!("{counter}");
    }
}
