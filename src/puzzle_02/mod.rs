use std::io::{stdin, Error};

pub fn get_input() -> Result<String, Error> {
    let mut i = String::new();
    stdin().read_line(&mut i)?;
    Ok(i.trim().into())
}
