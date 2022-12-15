use std::{collections::HashMap, fs, io, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    ParseError(ParseIntError),
    InvalidPosition(String),
}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::ReadInput(error)
    }
}
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseError(error)
    }
}

#[derive(Debug)]
pub enum Material {
    Rock,
    Sand,
}

#[derive(Debug)]
pub enum SandState {
    Fall,
    Rest,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    pub x: usize,
    pub y: usize,
}
impl FromStr for Position {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut list = input.split(',').into_iter();
        let x = list.next().ok_or(Error::InvalidPosition(input.into()))?.parse()?;
        let y = list.next().ok_or(Error::InvalidPosition(input.into()))?.parse()?;
        Ok(Self { x, y })
    }
}
impl Position {
    pub fn down(&mut self) -> bool {
        self.y += 1;
        true
    }

    pub fn left(&mut self) -> bool {
        self.x -= 1;
        true
    }

    pub fn right(&mut self) -> bool {
        self.x += 2;
        true
    }
}

#[derive(Debug)]
struct SandUnit {
    pub position: Position,
    pub state: SandState,
}
impl SandUnit {
    pub fn drop(cave: &Cave) -> Self {
        let mut unit = Self {
            position: Position { x: 500, y: 0 },
            state: SandState::Fall,
        };
        loop {
            match unit.state {
                SandState::Fall => unit.drop_step(cave),
                _ => break,
            }
        }
        unit
    }

    fn drop_step(&mut self, cave: &Cave) {
        if let SandState::Fall = self.state {
            let mut pos = self.position;
            if pos.y > cave.bottom {
                self.state = SandState::Rest;
            } else if pos.down() && cave.fill.get(&pos).is_none() {
                self.position = pos;
            } else if pos.left() && cave.fill.get(&pos).is_none() {
                self.position = pos;
            } else if pos.right() && cave.fill.get(&pos).is_none() {
                self.position = pos;
            } else {
                self.state = SandState::Rest;
            }
        }
    }
}

#[derive(Debug)]
pub struct Cave {
    fill: HashMap<Position, Material>,
    bottom: usize,
}
impl FromStr for Cave {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut cave = Self {
            fill: HashMap::new(),
            bottom: 0,
        };
        for line in input.split('\n') {
            let mut list = line.split(" -> ");
            if let Some(Ok(mut prev)) = list.next().map(|p| Position::from_str(p)) {
                while let Some(Ok(next)) = list.next().map(|p| Position::from_str(p)) {
                    for x in prev.x.min(next.x)..=prev.x.max(next.x) {
                        for y in prev.y.min(next.y)..=prev.y.max(next.y) {
                            cave.fill.insert(Position { x, y }, Material::Rock);
                            cave.bottom = cave.bottom.max(y);
                        }
                    }
                    prev = next;
                }
            }
        }
        Ok(cave)
    }
}
impl Cave {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        let input = fs::read_to_string(path)?;
        Ok(Self::from_str(&input)?)
    }

    pub fn pour_sand(&mut self) {
        loop {
            let unit = SandUnit::drop(&self);
            self.fill.insert(unit.position, Material::Sand);
            if unit.position.y == 0 {
                break;
            }
        }
    }

    pub fn count_sand_units(&self) -> usize {
        self.fill
            .iter()
            .filter(|(_, unit)| {
                if let Material::Sand = unit {
                    true
                } else {
                    false
                }
            })
            .count()
    }
}

fn main() {
    let mut cave = Cave::load_from("data/input.txt").unwrap();
    cave.pour_sand();
    println!("{} sand units are rest in the cave", cave.count_sand_units());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut cave = Cave::load_from("data/test.txt").unwrap();
        cave.pour_sand();
        assert_eq!(cave.count_sand_units(), 93);
    }
}
