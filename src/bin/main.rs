use std::io::Error;
use exercise_rust::{double_it, allocate, verify, StdInReader, overflow};

fn main() -> Result<(), Error> {
    let reader = StdInReader{};
    println!("{}", verify(&reader)?);

    println!("{}", allocate());

    println!("{:.1}", double_it(3));
    println!("{:.1}", double_it(3.1));

    overflow();
    Ok(())
}
