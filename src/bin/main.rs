use std::io::Error;
use exercise_rust::{double_it, allocate, get_input};

fn main() -> Result<(), Error> {
    if get_input()? == "5" {
        println!("Correct");
    } else {
        println!("Wrong");
    }

    println!("{}", allocate());

    println!("{:.1}", double_it(3));
    println!("{:.1}", double_it(3.1));
    Ok(())
}
