use std::fs;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Supply(u32);
impl Supply {
    pub fn from(input: &str) -> Option<Self> {
        Some(Self(input.parse().ok()?))
    }
    
    pub fn calories(&self) -> u32 {
        self.0
    }
}

/// An elf carrying some supplies
/// ```
/// use aoc2022::Elf;
///
/// let elf = Elf::from("1000\n2000\n3000").unwrap();
/// assert_eq!(elf.calories(), 6000);
/// ```
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Elf(Vec<Supply>);
impl Elf {
    pub fn from(input: &str) -> Option<Self> {
        let mut supplies: Vec<Supply> = Vec::new();
        for s in input.split('\n') { supplies.push(Supply::from(s)?); }
        Some(Self(supplies))
    }

    pub fn calories(&self) -> u32 {
        self.0.iter().fold(0, |a, supply| a + supply.calories())
    }
}

/// A group of elves
/// ```
/// use aoc2022::Elves;
///
/// let elves = Elves::from("1\n2\n3\n\n4\n\n5\n6\n\n7\n8\n9\n\n10").unwrap();
/// println!("{:?}", elves);
/// assert_eq!(elves.calories_carried_by_top(1), 24);
/// assert_eq!(elves.calories_carried_by_top(3), 45);
/// ```
#[derive(Debug)]
pub struct Elves(Vec<Elf>);
impl Elves {
    pub fn from(input: &str) -> Option<Self> {
        let mut elves: Vec<Elf> = Vec::new();
        for s in input.split("\n\n") { elves.push(Elf::from(s)?); }
        elves.sort_by(|a, b| b.calories().cmp(&a.calories()));
        Some(Self(elves))
    }

    pub fn load_from(path: &str) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;
        Self::from(&data)
    }

    pub fn calories_carried_by_top(&self, count: usize) -> u32 {
        self.0.iter().take(count).fold(0, |a, elf| a + elf.calories())
    }
}
