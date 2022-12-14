use std::{fs, io, num::ParseIntError, str::FromStr, collections::BinaryHeap};

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

#[derive(Debug, Default, PartialEq, Eq, Ord, PartialOrd)]
struct Elf { calories: u32 }
impl FromStr for Elf {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut elf = Self::default();
        for line in input.lines() {
            elf.calories += line.parse::<u32>()?;
        }
        Ok(elf)
    }
}

#[derive(Debug, Default)]
struct Elves(BinaryHeap<Elf>);
impl FromStr for Elves {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut elves = Self::default();
        for s in input.split("\n\n") {
            elves.0.push(Elf::from_str(s)?);
        }
        Ok(elves)
    }
}
impl Iterator for Elves {
    type Item = Elf;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
impl Elves {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        let data = fs::read_to_string(path)?;
        Self::from_str(&data)
    }
    
    pub fn top_calories(&mut self, size: usize) -> u32 {
        self.take(size).map(|elf| elf.calories).sum()
    }
}

fn main() {
    let mut elves = Elves::load_from("data/input.txt").unwrap();
    let calories = elves.top_calories(1);
    println!("The strongest elf carries {} calories", calories);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut elves = Elves::load_from("data/test.txt").unwrap();
        let calories = elves.top_calories(1);
        assert_eq!(calories, 24000);
    }
}
