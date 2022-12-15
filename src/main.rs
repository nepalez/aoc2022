use std::{fs, io, num::ParseIntError, str::FromStr};
use std::collections::HashMap;
use bitmaps::Bitmap;

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    Parse(ParseIntError),
    InvalidCoords,
}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::ReadInput(error)
    }
}
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::Parse(error)
    }
}

#[derive(Debug, Default, Clone)]
struct Crystal(HashMap<(usize, usize, usize), Bitmap<6>>);
impl FromStr for Crystal {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut crystal = Self::default();
        for line in input.lines() {
            let mut items = line.split(',');
            let x = items.next().ok_or(Error::InvalidCoords)?.parse::<usize>()?;
            let y = items.next().ok_or(Error::InvalidCoords)?.parse::<usize>()?;
            let z = items.next().ok_or(Error::InvalidCoords)?.parse::<usize>()?;
            let mut cube = Bitmap::new();
            for i in 0..6 {
                cube.set(i, true);
            }
            crystal.0.insert((x, y, z), cube);
        }
        Ok(crystal)
    }
}
impl Crystal {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        Self::from_str(&fs::read_to_string(path)?)
    }
    
    pub fn calculate(&mut self) {
        let crystal = self.0.clone();
        for (&(x, y, z), cube) in self.0.iter_mut() {
            if x > 0 && crystal.contains_key(&(x - 1, y, z)) {
                cube.set(0, false);
            }
            if crystal.contains_key(&(x + 1, y, z)) {
                cube.set(1, false);
            }
            if y > 0 && crystal.contains_key(&(x, y - 1, z)) {
                cube.set(2, false);
            }
            if crystal.contains_key(&(x, y + 1, z)) {
                cube.set(3, false);
            }
            if z > 0 && crystal.contains_key(&(x, y, z - 1)) {
                cube.set(4, false);
            }
            if crystal.contains_key(&(x, y, z + 1)) {
                cube.set(5, false);
            }
        }
    }
    
    pub fn sides(&self) -> usize {
        self.0.iter().map(|(_, cube)| cube.len()).sum()
    }
}

fn main() {
    let mut crystal = Crystal::load_from("data/input.txt").unwrap();
    crystal.calculate();
    println!("The crystal has {} sides", crystal.sides());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut crystal = Crystal::load_from("data/test.txt").unwrap();
        crystal.calculate();
        assert_eq!(crystal.sides(), 64);
    }
}
