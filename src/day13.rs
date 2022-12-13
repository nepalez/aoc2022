use serde::Deserialize;
use std::{
    cmp::Ordering,
    fs,
    str::FromStr,
    vec::IntoIter,
};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Package {
    Number(u32),
    Array(Box<Vec<Package>>),
}

impl FromStr for Package {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = &format!("[{}]", s);
        if let Ok(signal) = serde_json::from_str(input) {
            Ok(signal)
        } else {
            Err(())
        }
    }
}

impl From<Vec<u32>> for Package {
    fn from(input: Vec<u32>) -> Self {
        Self::Array(Box::new(input.iter().map(|&number| Self::Number(number)).collect()))
    }
}

impl PartialOrd<Self> for Package {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Package::Number(left), &Package::Number(right)) => left.partial_cmp(&right),
            (&Package::Number(left), &Package::Array(ref _right)) => {
                Self::from(vec![left]).partial_cmp(other)
            }
            (&Package::Array(ref _left), &Package::Number(right)) => {
                self.partial_cmp(&Self::from(vec![right]))
            }
            (&Package::Array(ref left), &Package::Array(ref right)) => {
                let mut left = left.iter();
                let mut right = right.iter();
                loop {
                    match (left.next(), right.next()) {
                        (None, None) => return Some(Ordering::Equal),
                        (Some(_), None) => return Some(Ordering::Greater),
                        (None, Some(_)) => return Some(Ordering::Less),
                        (Some(l), Some(r)) => {
                            let result = l.partial_cmp(r)?;
                            if result != Ordering::Equal {
                                return Some(result);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Packages(Package, Package);

impl FromStr for Packages {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.split('\n').into_iter();
        if let Some(left) = data.next() {
            if let Ok(left) = Package::from_str(left) {
                if let Some(right) = data.next() {
                    if let Ok(right) = Package::from_str(right) {
                        return Ok(Self(left, right));
                    }
                }
            }
        }
        Err(())
    }
}

impl IntoIterator for Packages {
    type Item = Package;
    type IntoIter = IntoIter<Package>;

    fn into_iter(self) -> Self::IntoIter {
        if self.0 <= self.1 {
            vec![self.0, self.1]
        } else {
            vec![self.1, self.0]
        }
        .into_iter()
    }
}

impl Packages {
    pub fn is_ordered(&self) -> bool {
        self.0 <= self.1
    }
}

#[derive(Debug)]
pub struct Signal(Vec<Packages>);

impl FromStr for Signal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = Vec::new();
        for part in s.split("\n\n") {
            if let Ok(pair) = Packages::from_str(part) {
                output.push(pair);
            } else {
                return Err(());
            }
        }
        Ok(Self(output))
    }
}

impl Into<Vec<Package>> for Signal {
    fn into(self) -> Vec<Package> {
        let mut packages: Vec<Package> = self.0.into_iter().flat_map(|pair| pair).collect();
        packages.push(Package::from_str("[[2]]").unwrap());
        packages.push(Package::from_str("[[6]]").unwrap());
        packages.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        packages
    }
}

impl Signal {
    pub fn load_from(path: &str) -> Option<Self> {
        let input = fs::read_to_string(path).ok()?;
        Some(Self::from_str(&input).ok()?)
    }

    pub fn sum_right_indexes(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, pair)| pair.is_ordered())
            .map(|(index, _)| 1 + index)
            .fold(0, |a, i| a + i)
    }

    pub fn decoder_key(self) -> Option<usize> {
        let packages = self.into();

        let signal = Package::from_str("[[2]]").ok()?;
        let first = 1 + packages.iter().position(|s| s == &signal)?;

        let signal = Package::from_str("[[6]]").ok()?;
        let second = 1 + packages.iter().position(|s| s == &signal)?;

        Some(first * second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let signal = Signal::load_from("data/13_test.in").unwrap();
        assert_eq!(signal.sum_right_indexes(), 13);
        assert_eq!(signal.decoder_key(), Some(140));
    }
}
