use regex;
use std::{io, num};

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    ParseInt(num::ParseIntError),
    ParseRegex(regex::Error),
    UnexpectedMaterial(String),
    BlueprintError(usize),
}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::ReadInput(error)
    }
}
impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}
impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Self::ParseRegex(error)
    }
}
