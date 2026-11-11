use std::io::Error;
use exercise_rust::double_it;

fn main() -> Result<(), Error> {
    println!("{:.1}", double_it(3));
    println!("{:.1}", double_it(3.1));
    Ok(())
}
