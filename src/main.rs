use std::{fs, io, num::ParseIntError, str::FromStr};

#[derive(Debug)]
enum Error {
    ReadInput(io::Error),
    ParseCalories(ParseIntError),
}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::ReadInput(error)
    }
}
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseCalories(error)
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {}
}
