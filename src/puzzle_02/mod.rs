use std::io::{stdin, Error};

pub fn get_input() -> Result<String, Error> {
    let mut i = String::new();
    stdin().read_line(&mut i)?;
    Ok(i.trim().into())
}

pub fn verify(input: &str) -> &'static str {
    if input == "5" {
        "Correct"
    } else {
        "Wrong"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify() {
        assert_eq!(verify("4"), "Wrong");
        assert_eq!(verify("5"), "Correct");
        assert_eq!(verify("6"), "Wrong");
    }
}
