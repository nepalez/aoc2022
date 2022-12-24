use crate::Direction::{E, N, NE, NW, S, SE, SW, W};
use regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::{fs, io, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    ParseInt(ParseIntError),
    ParseRegex(regex::Error),
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
impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Self::ParseRegex(error)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    W,
    E,
    NE,
    NW,
    SE,
    SW,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Elf(i32, i32);
impl Elf {
    pub fn side(&self, direction: &Direction) -> Vec<Self> {
        match direction {
            &N => vec![self.go(&N), self.go(&NW), self.go(&NE)],
            &S => vec![self.go(&S), self.go(&SW), self.go(&SE)],
            &E => vec![self.go(&E), self.go(&NE), self.go(&SE)],
            &W => vec![self.go(&W), self.go(&NW), self.go(&SW)],
            _ => vec![],
        }
    }

    pub fn go(&self, direction: &Direction) -> Self {
        match direction {
            &N => Self(self.0 - 1, self.1),
            &S => Self(self.0 + 1, self.1),
            &W => Self(self.0, self.1 - 1),
            &E => Self(self.0, self.1 + 1),
            &SE => Self(self.0 + 1, self.1 + 1),
            &SW => Self(self.0 + 1, self.1 - 1),
            &NE => Self(self.0 - 1, self.1 + 1),
            &NW => Self(self.0 - 1, self.1 - 1),
        }
    }
    
    pub fn neighbours(&self) -> Vec<Self> {
        vec![
            self.go(&N),
            self.go(&S),
            self.go(&W),
            self.go(&E),
            self.go(&NW),
            self.go(&NE),
            self.go(&SW),
            self.go(&SE),
        ]
    }
}

#[derive(Debug)]
struct DirectionOrder {
    list: VecDeque<Direction>,
}
impl Iterator for DirectionOrder {
    type Item = VecDeque<Direction>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.list.clone();
        let dir = self.list.pop_front()?;
        self.list.push_back(dir);
        Some(item)
    }
}
impl DirectionOrder {
    pub fn new() -> Self {
        Self {
            list: VecDeque::from([N, S, W, E]),
        }
    }
}

#[derive(Debug)]
struct Elves {
    list: HashSet<Elf>,
    directions: DirectionOrder,
}
impl FromStr for Elves {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut list = HashSet::default();
        for (y, line) in s.lines().enumerate() {
            for (x, item) in line.chars().enumerate() {
                if item == '#' {
                    list.insert(Elf(y as i32, x as i32));
                }
            }
        }
        Ok(Self { list, directions: DirectionOrder::new() })
    }
}
impl From<&Elves> for String {
    fn from(elves: &Elves) -> Self {
        let mut output = String::from("---\n");

        let (first, last) = elves.corners();
        for r in first.0..=last.0 {
            for c in first.1..=last.1 {
                if elves.list.contains(&Elf(r, c)) {
                    output.push('#');
                } else {
                    output.push('.');
                }
            }
            output.push('\n');
        }

        output
    }
}
impl Display for Elves {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self))
    }
}
impl Elves {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        Self::from_str(&fs::read_to_string(path)?)
    }

    pub fn count_rounds(&mut self) -> usize {
        let mut last = 1;
        while self.run() {
            last += 1;
        }
        last
    }
    
    pub fn goto_round(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.run();
        }
    }

    pub fn empty_ground(&self) -> i32 {
        let corners = self.corners();
        let square = (corners.1 .0 - corners.0 .0 + 1) * (corners.1 .1 - corners.0 .1 + 1);
        square - self.list.len() as i32
    }

    /// Run to new positions and return if a final position is not reached yet
    pub fn run(&mut self) -> bool {
        let mut final_position_reached = true;
        let directions = self.directions.next().unwrap();
        
        // find proposed positions: new_position -> [old_position]
        let mut new_positions: HashMap<Elf, Vec<Elf>> = HashMap::new();
        for old in self.list.iter() {
            let mut new = old.clone();

            // check if elf should keep the rest
            let mut should_rest = true;
            for pos in old.neighbours() {
                if self.list.contains(&pos) {
                    final_position_reached = false;
                    should_rest = false;
                }
            }

            // propose a new position
            if !should_rest {
                for direction in directions.iter() {
                    let mut can_move = true;
                    for pos in old.side(direction) {
                        if self.list.contains(&pos) {
                            can_move = false;
                            break;
                        }
                    }
                    if can_move {
                        new = old.go(direction);
                        break;
                    }
                }
            }

            // register the proposed position
            let mut pretending = new_positions.get(&new).unwrap_or(&Vec::new()).clone();
            pretending.push(old.clone());
            new_positions.insert(new, pretending);
        }

        // if no elf has neighbours, then stop in the current position
        if final_position_reached {
            return false;
        }

        // otherwise define new positions by checking pretending elves
        let mut list: HashSet<Elf> = HashSet::new();
        for (new, olds) in new_positions {
            if olds.len() == 1 {
                // use new position if only one pretendent exists
                list.insert(new);
            } else {
                // otherwise keep the previous position
                for old in olds {
                    list.insert(old);
                }
            }
        }

        self.list = list;
        true
    }

    fn corners(&self) -> (Elf, Elf) {
        let mut corners = (Elf(i32::MAX, i32::MAX), Elf(i32::MIN, i32::MIN));
        for &e in self.list.iter() {
            corners.0 .0 = corners.0 .0.min(e.0);
            corners.0 .1 = corners.0 .1.min(e.1);
            corners.1 .0 = corners.1 .0.max(e.0);
            corners.1 .1 = corners.1 .1.max(e.1);
        }
        corners
    }
}

fn main() {
    let mut elves = Elves::load_from("data/input.txt").unwrap();
    println!("The first sleepy round is: {}", elves.count_rounds());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut elves = Elves::load_from("data/test2.txt").unwrap();
        assert_eq!(elves.count_rounds(), 20);
    }
}
