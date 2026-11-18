use exercise_rust::{
    Degrees, Radians, StdInReader, allocate, double_it, overflow, sort_float, verify,
};
use std::f32::{INFINITY, NAN};
use std::io::Error;

fn main() -> Result<(), Error> {
    let target: Vec<f32> = vec![NAN, INFINITY, 3.1, 1.2, 4.5, 0.3, 0.3];
    if let Ok(sorted) = sort_float(target) {
        println!("{:?}", sorted);
    } else {
        println!("Not sorted");
    }

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
