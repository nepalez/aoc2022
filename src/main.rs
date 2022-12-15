use std::{fs, io, num::ParseIntError, str::FromStr};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    ParseInt(ParseIntError),
    ParseRegex(regex::Error),
    InvalidNumber,
}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::ReadInput(error)
    }
}
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseInt(error)
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Snafu {
    pub size: usize,
    pub digits: Vec<i128>,
}
impl FromStr for Snafu {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits: Vec<i128> = vec![];
        for c in s.chars().rev() {
            digits.push(match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => Err(Error::InvalidNumber)?,
            });
        }
        let size = digits.len();
        Ok(Self { digits, size })
    }
}
impl Display for Snafu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self))
    }
}
impl From<&Snafu> for String {
    fn from(s: &Snafu) -> Self {
        let mut output = String::with_capacity(s.size);
        for &i in s.digits.iter() {
            output.push(match i {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => panic!(),
            });
        }
        output.chars().into_iter().rev().collect()
    }
}
impl From<&Snafu> for i128 {
    fn from(s: &Snafu) -> Self {
        let mut result = 0;
        for (pos, &num) in s.digits.iter().enumerate() {
            result += 5_i128.pow(pos as u32) * num;
        }
        result
    }
}
impl From<&i128> for Snafu {
    fn from(number: &i128) -> Self {
        let mut digits = vec![];
        let mut number = number.clone();
        while number != 0 {
            let (digit, item) = match number % 5 {
                0 => (0, '0'),
                1 => (1, '1'),
                2 => (2, '2'),
                3 => (-2, '='),
                4 => (-1, '-'),
                -1 => (-1, '-'),
                -2 => (-2, '='),
                -3 => (2, '2'),
                -4 => (1, '1'),
                _ => panic!(),
            };
            number = (number - digit) / 5;
            digits.push(digit);
        }
        let size = digits.len();
        Self { digits, size }
    }
}

#[derive(Debug, Default, Clone)]
struct Requirements(Vec<Snafu>);
impl Requirements {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        let mut output = Self::default();
        for line in fs::read_to_string(path)?.lines() {
            output.0.push(Snafu::from_str(line)?);
        }
        Ok(output)
    }
    
    pub fn sum_decimal(&self) -> i128 {
        let decimals: Vec<i128> = self.into();
        decimals.iter().sum()
    }
    
    pub fn sum_snafu(self) -> Snafu {
        Snafu::from(&self.sum_decimal().into())
    }
}
impl Into<Vec<i128>> for &Requirements {
    fn into(self) -> Vec<i128> {
        self.0.iter().map(|s| i128::from(s)).collect()
    }
}

fn main() {
    let requirements = Requirements::load_from("data/input.txt").unwrap();
    println!("SNAFU: {}", requirements.sum_snafu());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let requirements = Requirements::load_from("data/test.txt").unwrap();
        assert_eq!(requirements.sum_decimal(), 4890);
        assert_eq!(String::from(&requirements.sum_snafu()), String::from("2=-1=0"));
    }
}
