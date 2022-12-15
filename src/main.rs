use regex;
use std::{fs, io, num::ParseIntError, str::FromStr};

const WINDOW: i32 = 4000000;

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
    pub open_distance: i32,
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
        let open_distance = sensor.distance(&beacon);

        Ok(Self {
            sensor,
            open_distance,
        })
    }
}
impl Signal {
    pub fn open_interval(&self, row: i32) -> (i32, i32) {
        let dist = self.open_distance - (row - self.sensor.y).abs();
        let min = (self.sensor.x - dist).min(WINDOW).max(0);
        let max = (self.sensor.x + dist).min(WINDOW).max(0);
        (min, max)
    }
}

#[derive(Debug)]
struct CaveRow {
    hidden_space: Vec<(i32, i32)>,
}
impl CaveRow {
    pub fn new(area_width: i32) -> Self {
        Self {
            hidden_space: vec![(0, area_width)],
        }
    }

    pub fn remove(&mut self, new_interval: (i32, i32)) {
        if new_interval.0 > new_interval.1 {
            return;
        }
        let mut space = Vec::with_capacity(self.hidden_space.len() + 1);
        for &old_interval in self.hidden_space.iter() {
            if old_interval.1 < new_interval.0 || new_interval.1 < old_interval.0 {
                space.push(old_interval);
            } else {
                if new_interval.0 > old_interval.0 {
                    space.push((old_interval.0, new_interval.0 - 1));
                }
                if new_interval.1 < old_interval.1 {
                    space.push((new_interval.1 + 1, old_interval.1));
                }
            }
        }
        self.hidden_space = space;
    }

    pub fn distress_position(&self) -> Option<i32> {
        if self.hidden_space.len() == 1 {
            if let Some(&(min, max)) = self.hidden_space.iter().next() {
                if min == max {
                    return Some(min);
                }
            }
        }
        None
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

    pub fn distress_position(&self, area_width: i32) -> Option<(i32, i32)> {
        for y in 0..=WINDOW {
            let mut unknown = CaveRow::new(area_width);
            for signal in self.0.iter() {
                unknown.remove(signal.open_interval(y));
            }
            if let Some(x) = unknown.distress_position() {
                return Some((x, y));
            }
        }
        None
    }

    pub fn tuning_frequency(&self, area_width: i32) -> Option<u64> {
        let (x, y) = self.distress_position(area_width)?;
        Some(x as u64 * WINDOW as u64 + y as u64)
    }
}

fn main() {
    let device = Device::load_from("data/input.txt").unwrap();
    println!("Tuning frequency is {:?}", device.tuning_frequency(WINDOW));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let device = Device::load_from("data/test.txt").unwrap();
        assert_eq!(device.tuning_frequency(20), Some(56000011));
    }
}
