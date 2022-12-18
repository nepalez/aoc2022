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

#[derive(Debug, Default, Clone)]
struct Rocks {
    pub counter: usize,
    pub current: usize,
}
impl Iterator for Rocks {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        let form = match self.current {
            0 => RockForm::Minus,
            1 => RockForm::Plus,
            2 => RockForm::Corner,
            3 => RockForm::Stick,
            _ => RockForm::Rect,
        };
        self.counter += 1;
        self.current = self.counter % 5;
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

#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone, Eq)]
pub struct Chamber {
    pub height: usize,
    pub bottom: usize,
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

    // periodically we see the pattern like
    //
    // |     @ |
    // |##  @@@|
    // | ####@ | <-- bottom of the last rock before the freeze
    // |???????|
    //
    // that isolates the bottom, so those `?` can be safely removed.
    fn clean(&mut self, last_rock: Rock) -> bool {
        if last_rock.bottom < self.bottom + 2 {
            return false;
        }
        let bottom = last_rock.bottom + 3; // start checks here as a lower row

        // bottom - 1 should present otherwise the rock could not stop here
        for shift in 0..4 {
            let vertical = bottom - shift;
            if let Some(lower_row) = self.coords.get(&vertical) {
                if let Some(higher_row) = self.coords.get(&(vertical + 1)) {
                    let mut blocking = true;
                    for i in 0..7 {
                        if !lower_row.contains(&i) && !higher_row.contains(&i) {
                            blocking = false;
                        }
                    }
                    // remove isolated rows
                    if blocking {
                        for i in self.bottom..vertical {
                            self.coords.remove(&i);
                        }
                        self.coords.shrink_to_fit();
                        self.bottom = vertical;
                        return true;
                    }
                }
            }
        }
        false
    }
}
// Compare 2 chambers independently of their height
// at the moment the bottom is dropped.
impl PartialEq for Chamber {
    fn eq(&self, other: &Self) -> bool {
        let mut size = self.height.saturating_sub(self.bottom);
        if size != other.height.saturating_sub(other.bottom) {
            return false;
        }
        loop {
            let row = self.coords.get(&(size + self.bottom));
            let other_row = other.coords.get(&(size + other.bottom));
            if row != other_row {
                return false;
            }
            if size > 0 {
                size -= 1;
            } else {
                return true;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Process {
    chamber: Chamber,
    rocks: Rocks,
    wind: Wind,
    rock: Rock,
}
impl ToString for Process {
    fn to_string(&self) -> String {
        let rock_positions = self.rock.positions();

        let mut output = String::new();
        let mut i = self.rock.bottom + 3;
        while i > self.chamber.bottom.saturating_sub(1) {
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
            output.push_str(&format!(" : {}", i));
            output.push('\n');
            i = i.saturating_sub(1);
        }
        if self.chamber.bottom > 0 {
            output.push('|');
            for _ in 0..7 {
                output.push('?');
            }
            output.push('|');
        } else {
            output.push('+');
            for _ in 0..7 {
                output.push('_');
            }
            output.push('+');
        }
        output
    }
}
impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.chamber == other.chamber && self.rocks.current == other.rocks.current && self.wind.current == other.wind.current
    }
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

    pub fn restart(&mut self) {
        self.chamber = Chamber::new();
        self.wind.size = self.wind.pattern.len();
        self.wind.current = 0;
        self.rocks = Rocks::default();
        self.rock = self.rocks.next().unwrap();
        self.rock.place(self.chamber.height);
    }
    
    pub fn rocks_fallen(&self) -> usize {
        self.rocks.counter.saturating_sub(1)
    }
    
    pub fn chamber_height(&self) -> usize {
        self.chamber.height
    }
    
    pub fn cycle(&self) -> usize {
        self.wind.size * 5
    }

    /// fall given number of rocks
    pub fn fall_rocks(&mut self, number: usize) {
        for _ in 0..number {
            self.fall_next_rock();
        }
    }

    /// fall rocks until the next occurrence of the same pattern
    pub fn find_next_pattern(&mut self, process: &Self) {
        while &self != &process {
            self.find_next_clean();
        }
    }

    /// fall rocks until the bottom is cleaned
    pub fn find_next_clean(&mut self) {
        while !self.fall_next_rock() {}
    }

    pub fn print(&self) {
        for line in self.to_string().lines() {
            println!("{}", line);
        }
    }

    // wait until the rock is frozen and return if the chamber was cleaned
    fn fall_next_rock(&mut self) -> bool {
        let size = self.rocks.counter;
        let mut cleaned = false;
        while self.rocks.counter == size {
            self.blow();
            cleaned = self.fall();
        }
        cleaned
    }

    // handle the next jet's stroke
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

    // fall the rock and return if it was frozen and cleaned
    fn fall(&mut self) -> bool {
        let mut rock = self.rock.clone();
        rock.down();
        for (vertical, horizontal) in rock.positions() {
            if self.chamber.contains(vertical, horizontal) {
                return self.freeze();
            }
        }
        self.rock = rock;
        false
    }

    // freeze the rock and return if it was cleaned on this step
    fn freeze(&mut self) -> bool {
        for (vertical, horizontal) in self.rock.positions() {
            self.chamber.add(vertical, horizontal);
        }
        let cleaned = self.chamber.clean(self.rock.clone());
        self.rock = self.rocks.next().unwrap();
        self.rock.place(self.chamber.height);
        cleaned
    }
}

pub fn count_height(process: &mut Process) -> u128 {
    // go forward far enough (to a full wind x rocks cycle)
    process.restart();
    process.fall_rocks(process.cycle());

    // find the moment the chamber was cleaned inside the next cycle
    // this pattern should repeat
    process.find_next_clean();
    let pattern = process.clone();
    let rocks_start = process.rocks_fallen();
    let height_start = process.chamber_height();

    // find the next occurrence of the pattern
    process.fall_rocks(1);
    process.find_next_pattern(&pattern);
    let rocks_period = process.rocks_fallen() - rocks_start;
    let height_period = process.chamber_height() - height_start;

    // count the number of periods and remainder of rocks
    let full_periods = (1000000000000 - rocks_start as u128) / rocks_period as u128;
    let rocks_remainder = (1000000000000 - rocks_start as u128) % rocks_period as u128;

    // restart the process and iterate to the point equal to the end of the process
    process.restart();
    process.fall_rocks(rocks_start + rocks_remainder as usize);

    // count the expected hight
    process.chamber.height as u128 + full_periods * height_period as u128
}

fn main() {
    let mut process = Process::load_from("data/input.txt").unwrap();
    println!("{}", count_height(&mut process));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut process = Process::load_from("data/test.txt").unwrap();
        assert_eq!(count_height(&mut process), 1514285714288);
    }
}
