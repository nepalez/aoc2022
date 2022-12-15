use regex;
use std::{fs, io, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Error {
    FileError(io::Error),
    ParseError(ParseIntError),
    RegexError(regex::Error),
    InvalidPosition(String),
    InvalidSensor(String),
}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::FileError(error)
    }
}
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseError(error)
    }
}
impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Self::RegexError(error)
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    pub x: i32,
    pub y: i32,
}
impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rx = regex::Regex::new(r"x=(-?\d+), y=(-?\d+)")?;
        let caps = rx.captures(s).ok_or(Error::InvalidPosition(s.into()))?;
        let x = caps
            .get(1)
            .ok_or(Error::InvalidPosition(s.into()))?
            .as_str()
            .parse()?;
        let y = caps
            .get(2)
            .ok_or(Error::InvalidPosition(s.into()))?
            .as_str()
            .parse()?;
        Ok(Self { x, y })
    }
}
impl Position {
    pub fn distance(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Clone)]
struct Signal {
    pub sensor: Position,
    pub beacon: Position,
    pub empty_dist: i32,
}
impl FromStr for Signal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rx = regex::Regex::new("Sensor at ([^:]+): closest beacon is at ([^:]+)")?;
        let caps = rx.captures(s).ok_or(Error::InvalidSensor(s.into()))?;
        let s = caps.get(1).ok_or(Error::InvalidSensor(s.into()))?.as_str();
        let b = caps.get(2).ok_or(Error::InvalidSensor(s.into()))?.as_str();

        let sensor = Position::from_str(s)?;
        let beacon = Position::from_str(b)?;
        let empty_dist = sensor.distance(&beacon);

        Ok(Self {
            sensor,
            beacon,
            empty_dist,
        })
    }
}
impl Signal {
    pub fn empty_interval(&self, row: i32) -> (i32, i32) {
        let dist = self.empty_dist - (row - self.sensor.y).abs();

        let mut min = self.sensor.x - dist;
        let mut max = self.sensor.x + dist;
        if self.beacon.y == row {
            if self.beacon.x == min {
                min += 1;
            } else {
                max -= 1;
            }
        }

        (min, max)
    }
}

#[derive(Debug, Default)]
struct CaveRow {
    empty_space: Vec<(i32, i32)>,
}
impl CaveRow {
    pub fn add(&mut self, mut new_range: (i32, i32)) {
        if new_range.0 > new_range.1 {
            return;
        }
        let mut space = Vec::with_capacity(self.empty_space.len() + 1);
        for &old_range in self.empty_space.iter() {
            if old_range.1 < new_range.0 || new_range.1 < old_range.0 {
                // if the range doesn't intersects with a new one, keep it
                space.push(old_range);
            } else {
                // otherwise, union ranges
                new_range.0 = new_range.0.min(old_range.0);
                new_range.1 = new_range.1.max(old_range.1);
            }
        }
        space.push(new_range);
        self.empty_space = space;
    }

    pub fn empty_size(&self) -> i32 {
        self.empty_space
            .iter()
            .map(|&(start, end)| end - start + 1)
            .sum()
    }
}

#[derive(Debug, Default)]
struct Device(Vec<Signal>);
impl FromStr for Device {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut device = Self::default();
        for line in s.lines() {
            device.0.push(Signal::from_str(line)?);
        }
        Ok(device)
    }
}
impl Device {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        let input = fs::read_to_string(path)?;
        Self::from_str(&input)
    }

    pub fn empty_space(&self, row_index: i32) -> CaveRow {
        let mut row = CaveRow::default();
        for signal in self.0.iter() {
            row.add(signal.empty_interval(row_index));
        }
        row
    }
}

fn main() {
    let device = Device::load_from("data/input.txt").unwrap();
    let row = 2000000;
    let size = device.empty_space(row).empty_size();
    println!("The empty size in the row #{} is at least {}", row, size);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let device = Device::load_from("data/test.txt").unwrap();
        assert_eq!(device.empty_space(10).empty_size(), 26);
    }
}
