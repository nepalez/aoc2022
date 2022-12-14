use crate::Error::InvalidRound;
use std::io::BufRead;
use std::{fs, io, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    ParseCalories(ParseIntError),
    InvalidRound(String),
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

#[derive(Debug)]
pub struct Round((char, char));
impl FromStr for Round {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut data = input.chars();
        let left = data.next().ok_or(InvalidRound(input.into()))?;
        data.next();
        let right = data.next().ok_or(InvalidRound(input.into()))?;
        Ok(Self((left, right)))
    }
}
impl Round {
    pub fn score(&self) -> u32 {
        match self.0 {
            ('B', 'X') => 1, // PR
            ('C', 'Y') => 2, // SP
            ('A', 'Z') => 3, // RS
            ('A', 'X') => 4, // RR
            ('B', 'Y') => 5, // PP
            ('C', 'Z') => 6, // SS
            ('C', 'X') => 7, // SR
            ('A', 'Y') => 8, // RP
            ('B', 'Z') => 9, // PS
            _ => panic!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Game(Vec<Round>);
impl FromStr for Game {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data = input.split('\n');
        let mut game: Vec<Round> = Vec::new();
        for round in data {
            game.push(Round::from_str(round)?)
        }
        Ok(Self(game))
    }
}
impl Game {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        let mut game = Self::default();
        let file = fs::File::open(path)?;
        for line in io::BufReader::new(file).lines() {
            game.0.push(Round::from_str(&line?)?);
        }
        Ok(game)
    }

    pub fn score(&self) -> u32 {
        self.0.iter().fold(0, |a, r| a + r.score())
    }
}

fn main() {
    let game = Game::load_from("data/input.txt").unwrap();
    println!("The expected score is {}", game.score());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let game = Game::load_from("data/test.txt").unwrap();
        assert_eq!(game.score(), 15);
    }
}
