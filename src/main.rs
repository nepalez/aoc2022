use std::collections::{HashMap, HashSet};
use std::{fs, io, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    Parse(ParseIntError),
    InvalidChar,
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

#[derive(Debug, Default)]
struct Chamber {
    pub height: usize,
    coords: HashMap<usize, HashSet<usize>>,
}
impl Chamber {
    pub fn new() -> Self {
        let mut chamber = Self::default();
        for i in 0..7 {
            chamber.add(0, i);
        }
        chamber
    }

    pub fn add(&mut self, vertical: usize, horizontal: usize) {
        if !self.coords.contains_key(&vertical) {
            self.coords.insert(vertical, HashSet::with_capacity(7));
        }
        self.coords.get_mut(&vertical).unwrap().insert(horizontal);
        self.height = self.height.max(vertical);
    }

    pub fn contains(&self, vertical: usize, horizontal: usize) -> bool {
        if let Some(row) = self.coords.get(&vertical) {
            row.contains(&horizontal)
        } else {
            false
        }
    }

    pub fn flat(&self) -> bool {
        let top_row = self.coords.get(&self.height).unwrap();
        for i in 0..7 {
            if !top_row.contains(&i) {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum RockForm {
    Minus,
    Plus,
    Corner,
    Stick,
    Rect,
}

#[derive(Debug, Clone)]
struct Rock {
    form: RockForm,
    bottom: usize,
    left: usize,
}
impl Rock {
    pub fn new(form: RockForm) -> Self {
        Self {
            form,
            bottom: 4,
            left: 2,
        }
    }

    pub fn place(&mut self, chamber_height: usize) {
        self.bottom += chamber_height;
    }

    pub fn left(&mut self) {
        self.left = self.left.saturating_sub(1);
    }

    pub fn right(&mut self) {
        self.left += 1;
    }

    pub fn down(&mut self) {
        self.bottom = self.bottom.saturating_sub(1);
    }

    pub fn positions(&self) -> Vec<(usize, usize)> {
        match self.form {
            RockForm::Minus => vec![
                (self.bottom, self.left),
                (self.bottom, self.left + 1),
                (self.bottom, self.left + 2),
                (self.bottom, self.left + 3),
            ],
            RockForm::Plus => vec![
                (self.bottom + 2, self.left + 1),
                (self.bottom + 1, self.left),
                (self.bottom + 1, self.left + 1),
                (self.bottom + 1, self.left + 2),
                (self.bottom, self.left + 1),
            ],
            RockForm::Corner => vec![
                (self.bottom + 2, self.left + 2),
                (self.bottom + 1, self.left + 2),
                (self.bottom, self.left),
                (self.bottom, self.left + 1),
                (self.bottom, self.left + 2),
            ],
            RockForm::Stick => vec![
                (self.bottom, self.left),
                (self.bottom + 1, self.left),
                (self.bottom + 2, self.left),
                (self.bottom + 3, self.left),
            ],
            RockForm::Rect => vec![
                (self.bottom, self.left),
                (self.bottom, self.left + 1),
                (self.bottom + 1, self.left),
                (self.bottom + 1, self.left + 1),
            ],
        }
    }
}

#[derive(Debug, Default)]
struct Rocks {
    pub counter: usize,
}
impl Iterator for Rocks {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        let form = match self.counter % 5 {
            0 => RockForm::Minus,
            1 => RockForm::Plus,
            2 => RockForm::Corner,
            3 => RockForm::Stick,
            _ => RockForm::Rect,
        };
        self.counter += 1;
        Some(Rock::new(form))
    }
}

#[derive(Debug, Clone)]
enum Stroke {
    Left,
    Right,
}
impl TryFrom<char> for Stroke {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Stroke::Left),
            '>' => Ok(Stroke::Right),
            _ => Err(Error::InvalidChar),
        }
    }
}

#[derive(Debug, Default)]
struct Wind {
    pattern: Vec<Stroke>,
    size: usize,
    current: usize,
}
impl FromStr for Wind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut wind = Self::default();
        for c in s.chars() {
            wind.pattern.push(Stroke::try_from(c)?);
            wind.size += 1;
        }
        Ok(wind)
    }
}
impl Iterator for Wind {
    type Item = Stroke;

    fn next(&mut self) -> Option<Self::Item> {
        let stroke = self.pattern.get(self.current).map(|s| s.clone());
        self.current = (self.current + 1) % self.size;
        stroke
    }
}

#[derive(Debug)]
pub struct Process {
    chamber: Chamber,
    rocks: Rocks,
    wind: Wind,
    rock: Rock,
}
impl Process {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        let chamber = Chamber::new();
        let wind = Wind::from_str(&fs::read_to_string(path)?)?;
        let mut rocks = Rocks::default();
        let mut rock = rocks.next().unwrap();
        rock.place(chamber.height);

        Ok(Self {
            chamber,
            wind,
            rocks,
            rock,
        })
    }

    pub fn fall_rocks(&mut self, limit: usize) -> usize {
        while self.rocks.counter < (limit + 1) {
            self.next();
        }
        self.chamber.height
    }

    pub fn print(&self) {
        for line in self.to_string().lines() {
            println!("{}", line);
        }
    }

    fn next(&mut self) {
        self.blow();
        self.fall();
    }

    fn blow(&mut self) {
        let mut rock = self.rock.clone();
        match self.wind.next().unwrap() {
            Stroke::Left => rock.left(),
            Stroke::Right => rock.right(),
        }
        for (vertical, horizontal) in rock.positions() {
            if horizontal > 6 || self.chamber.contains(vertical, horizontal) {
                return;
            }
        }
        self.rock = rock;
    }

    fn fall(&mut self) {
        let mut rock = self.rock.clone();
        rock.down();
        for (vertical, horizontal) in rock.positions() {
            if self.chamber.contains(vertical, horizontal) {
                return self.freeze();
            }
        }
        self.rock = rock;
    }

    fn freeze(&mut self) {
        for (vertical, horizontal) in self.rock.positions() {
            self.chamber.add(vertical, horizontal);
        }
        self.rock = self.rocks.next().unwrap();
        self.rock.place(self.chamber.height);
    }
}
impl ToString for Process {
    fn to_string(&self) -> String {
        let rock_positions = self.rock.positions();

        let mut output = String::new();
        let mut i = self.rock.bottom + 3;
        while i > 0 {
            output.push('|');
            for x in 0..7 {
                if self.chamber.contains(i, x) {
                    output.push('#');
                } else if rock_positions.contains(&(i, x)) {
                    output.push('@');
                } else {
                    output.push('.');
                }
            }
            output.push('|');
            output.push('\n');
            i = i.saturating_sub(1);
        }
        output.push('+');
        for _ in 0..7 {
            output.push('-');
        }
        output.push('+');
        output
    }
}

fn main() {
    let mut process = Process::load_from("data/input.txt").unwrap();
    let chamber_height = process.fall_rocks(2022);
    println!(
        "The chamber height after 2022 rocks fallen is {}",
        chamber_height
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut process = Process::load_from("data/test.txt").unwrap();

        let mut chamber_height = process.fall_rocks(1);
        assert_eq!(chamber_height, 1);

        chamber_height = process.fall_rocks(2);
        assert_eq!(chamber_height, 4);

        chamber_height = process.fall_rocks(3);
        assert_eq!(chamber_height, 6);

        chamber_height = process.fall_rocks(4);
        assert_eq!(chamber_height, 7);

        chamber_height = process.fall_rocks(5);
        assert_eq!(chamber_height, 9);

        chamber_height = process.fall_rocks(6);
        assert_eq!(chamber_height, 10);

        chamber_height = process.fall_rocks(7);
        assert_eq!(chamber_height, 13);

        chamber_height = process.fall_rocks(8);
        assert_eq!(chamber_height, 15);

        chamber_height = process.fall_rocks(9);
        assert_eq!(chamber_height, 17);

        chamber_height = process.fall_rocks(10);
        assert_eq!(chamber_height, 17);

        chamber_height = process.fall_rocks(2022);
        assert_eq!(chamber_height, 3068);
    }
}
