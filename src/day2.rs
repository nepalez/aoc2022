use std::fs;

pub struct Input(pub Vec<(char, char)>);
impl Input {
    pub fn load_from(path: &str) -> Option<Input> {
        let data = fs::read_to_string(path).ok()?;

        let output: Vec<(char, char)> = data
            .split('\n')
            .map(|i| {
                let mut chars = i.chars();
                let a = chars.next().unwrap();
                chars.next();
                let b = chars.next().unwrap();
                (a, b)
            })
            .collect();

        Some(Self(output))
    }
}

#[derive(Debug)]
pub struct Round(u32);
impl Round {
    pub fn misinterpret(input: (char, char)) -> Option<Self> {
        let mut list = [
            ('B', 'X'), // PR
            ('C', 'Y'), // SP
            ('A', 'Z'), // RS
            ('A', 'X'), // RR
            ('B', 'Y'), // PP
            ('C', 'Z'), // SS
            ('C', 'X'), // SR
            ('A', 'Y'), // RP
            ('B', 'Z'), // PS
        ]
        .iter();
        Some(Round(list.position(|a| a == &input)? as u32 + 1))
    }

    pub fn interpret(input: (char, char)) -> Option<Self> {
        let mut list = [
            ('B', 'X'), // PR
            ('C', 'X'), // SP
            ('A', 'X'), // RS
            ('A', 'Y'), // RR
            ('B', 'Y'), // PP
            ('C', 'Y'), // SS
            ('C', 'Z'), // SR
            ('A', 'Z'), // RP
            ('B', 'Z'), // PS
        ]
        .iter();
        Some(Round(list.position(|a| a == &input)? as u32 + 1))
    }
}

/// Strategy of the Rock-Scissors-Paper game against an elf
/// ```
/// use aoc2022::{Input, Strategy};
///
/// let input = Input(Vec::from([
///   ('A', 'Y'),
///   ('B', 'X'),
///   ('C', 'Z'),
/// ]));
/// let strategy = Strategy::misinterpret(&input).unwrap();
/// assert_eq!(strategy.score(), 15);
///
/// let input = Input(Vec::from([
///   ('A', 'Y'),
///   ('B', 'X'),
///   ('C', 'Z'),
/// ]));
/// let strategy = Strategy::interpret(&input).unwrap();
/// assert_eq!(strategy.score(), 12);
/// ```
#[derive(Debug)]
pub struct Strategy(Vec<Round>);
impl Strategy {
    pub fn misinterpret(input: &Input) -> Option<Self> {
        let mut rounds: Vec<Round> = Vec::with_capacity(input.0.len());
        for i in input.0.iter().map(|&r| Round::misinterpret(r)) {
            if let Some(round) = i {
                rounds.push(round);
            } else {
                return None;
            }
        }
        Some(Self(rounds))
    }

    pub fn interpret(input: &Input) -> Option<Self> {
        let mut rounds: Vec<Round> = Vec::with_capacity(input.0.len());
        for i in input.0.iter().map(|&r| Round::interpret(r)) {
            if let Some(round) = i {
                rounds.push(round);
            } else {
                return None;
            }
        }
        Some(Self(rounds))
    }

    pub fn score(&self) -> u32 {
        self.0
            .iter()
            .map(|r| r.0)
            .fold(0, |result, score| result + score)
    }
}
