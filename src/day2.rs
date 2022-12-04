use std::fs;

#[derive(Debug)]
pub struct Round((char, char));
impl Round {
    pub fn from(input: &str) -> Option<Self> {
        let mut data = input.split(' ');
        let left = match data.next()? {
            "A" => Some('A'),
            "B" => Some('B'),
            "C" => Some('C'),
            _ => None,
        }?;
        let right = match data.next()? {
            "X" => Some('X'),
            "Y" => Some('Y'),
            "Z" => Some('Z'),
            _ => None,
        }?;
        Some(Self((left, right)))
    }

    pub fn wrong_score(&self) -> u32 {
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

    pub fn right_score(&self) -> u32 {
        match self.0 {
            ('B', 'X') => 1, // PR
            ('C', 'X') => 2, // SP
            ('A', 'X') => 3, // RS
            ('A', 'Y') => 4, // RR
            ('B', 'Y') => 5, // PP
            ('C', 'Y') => 6, // SS
            ('C', 'Z') => 7, // SR
            ('A', 'Z') => 8, // RP
            ('B', 'Z') => 9, // PS
            _ => panic!(),
        }
    }
}

/// Rock-Scissors-Paper game against an elf
/// ```
/// use aoc2022::Game;
///
/// let game = Game::from("A Y\nB X\nC Z").unwrap();
/// assert_eq!(game.wrong_score(), 15);
/// assert_eq!(game.right_score(), 12);
/// ```
#[derive(Debug)]
pub struct Game(Vec<Round>);
impl Game {
    pub fn from(input: &str) -> Option<Self> {
        let data = input.split('\n');
        let mut game: Vec<Round> = Vec::new();
        for round in data {
            game.push(Round::from(round)?)
        }
        Some(Self(game))
    }

    pub fn load_from(path: &str) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;
        Self::from(&data)
    }

    pub fn wrong_score(&self) -> u32 {
        self.0
            .iter()
            .fold(0, |acc, round| acc + round.wrong_score())
    }

    pub fn right_score(&self) -> u32 {
        self.0
            .iter()
            .fold(0, |acc, round| acc + round.right_score())
    }
}
