use exercise_rust::{Degrees, Radians, StdInReader, allocate, double_it, overflow, verify};
use std::io::Error;

fn main() -> Result<(), Error> {
    let degree = Degrees(180.0);
    let radian: Radians = degree.into();
    let degree2: Degrees = radian.into();
    println!("{}", degree2.0);

    let reader = StdInReader {};
    println!("{}", verify(&reader)?);

    println!("{}", allocate());

    println!("{:.1}", double_it(3));
    println!("{:.1}", double_it(3.1));

    overflow();
    Ok(())
}
