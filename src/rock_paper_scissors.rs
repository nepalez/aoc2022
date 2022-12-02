#[derive(Debug)]
pub struct Round(char, char);
impl Round {
    pub fn from(input: (char, char)) -> Option<Self> {
        let left = match input.0 {
            'A' => Some('R'),
            'B' => Some('P'),
            'C' => Some('S'),
            _ => None,
        }?;
        let right = match input.1 {
            'X' => Some('R'),
            'Y' => Some('P'),
            'Z' => Some('S'),
            _ => None,
        }?;
        Some(Self(left, right))
    }
    
    pub fn score(&self) -> u32 {
        match self {
            Round('P', 'R') => 1,
            Round('S', 'P') => 2,
            Round('R', 'S') => 3,
            Round('R', 'R') => 4,
            Round('P', 'P') => 5,
            Round('S', 'S') => 6,
            Round('S', 'R') => 7,
            Round('R', 'P') => 8,
            Round('P', 'S') => 9,
            _ => 0,
        }
    }
}

/// Strategy of the Rock-Scissors-Paper game against an elf
/// ```
/// use aoc2022::Strategy;
/// 
/// let input: Vec<(char, char)> = Vec::from([
///   ('A', 'Y'),
///   ('B', 'X'),
///   ('C', 'Z'),
/// ]);
/// let strategy = Strategy::new(input).unwrap();
/// assert_eq!(strategy.score(), 15);
/// ```
#[derive(Debug)]
pub struct Strategy(Vec<Round>);
impl Strategy {
    pub fn new(data: Vec<(char, char)>) -> Option<Self> {
        let mut rounds: Vec<Round> = Vec::with_capacity(data.len());
        for i in data.iter().map(|&r| Round::from(r)) {
            if let Some(round) = i {
                rounds.push(round);
            } else {
                return None;
            }
        }
        Some(Self(rounds))
    }
    
    pub fn score(&self) -> u32 {
        self.0.iter().map(|r| r.score()).fold(0, |result, score| result + score)
    }
}
