use std::io::Error;
use exercise_rust::{double_it, allocate, verify, get_input};

fn main() -> Result<(), Error> {
    println!("{}", verify(get_input()?.as_str()));

    println!("{}", allocate());

    println!("{:.1}", double_it(3));
    println!("{:.1}", double_it(3.1));
    Ok(())
}
