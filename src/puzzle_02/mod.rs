use std::io::{stdin, Error};

pub trait InputReader {
    fn read_line(&self) -> Result<String, Error>;
}

pub struct StdInReader;

impl InputReader for StdInReader {
    fn read_line(&self) -> Result<String, Error> {
        let mut i = String::new();
        stdin().read_line(&mut i)?;
        Ok(i.trim().into())
    }
}

pub fn verify(reader: &impl InputReader) -> Result<&'static str, Error> {
    let input = reader.read_line()?;
    if input == "5" {
        Ok("Correct")
    } else {
        Ok("Wrong")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestReader {
        input: String,
    }

    impl InputReader for TestReader {
        fn read_line(&self) -> Result<String, Error> {
            Ok(self.input.to_owned())
        }
    }

    #[test]
    fn test_verify() {
        let mut reader = TestReader{
            input: String::from(""),
        };
        assert_eq!(verify(&reader).unwrap(), "Wrong");

        reader.input = String::from("4");
        assert_eq!(verify(&reader).unwrap(), "Wrong");
        reader.input = String::from("5");
        assert_eq!(verify(&reader).unwrap(), "Correct");
        reader.input = String::from("6");
        assert_eq!(verify(&reader).unwrap(), "Wrong");
    }
}
