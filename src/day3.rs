use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

/// The item in the rucksack
/// ```
/// use aoc2022::Item;
///
/// let item = Item('p');
/// assert_eq!(item.score(), 16);
/// ```
#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Item(pub char);
impl Item {
    pub fn score(&self) -> u32 {
        let mut lower = 'a'..='z';
        let mut upper = 'A'..='Z';
        if let Some(p) = lower.position(|l| l == self.0) {
            p as u32 + 1
        } else {
            upper.position(|u| u == self.0).unwrap() as u32 + 27
        }
    }
}

#[derive(Debug, Clone)]
pub struct Items(Vec<Item>);
impl Items {
    pub fn from(input: &str) -> Self {
        let mut output = Vec::with_capacity(input.len());
        for c in input.chars() {
            output.push(Item(c))
        }
        Self(output)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn uniq_items(&self) -> HashSet<Item> {
        let mut collector: HashSet<Item> = HashSet::with_capacity(self.0.len());
        for i in self.0.iter() {
            collector.insert(i.clone());
        }
        collector
    }
}

#[derive(Debug)]
pub struct Group(pub Vec<Items>);
impl Group {
    pub fn badge(&self) -> Option<Item> {
        let group_size = self.0.len();
        let total_size = self.0.iter().fold(0, |a, i| a + i.len());

        let mut counter: HashMap<Item, usize> = HashMap::with_capacity(total_size);
        for items in self.0.iter() {
            for i in items.uniq_items() {
                if let Some(c) = counter.get(&i) {
                    counter.insert(i, c + 1);
                } else {
                    counter.insert(i, 1);
                }
            }
        }

        for (item, c) in counter {
            if c == group_size {
                return Some(item);
            }
        }
        None
    }

    pub fn score(&self) -> Option<u32> {
        Some(self.badge()?.score())
    }
}

/// The rucksack with left and right compartments
/// ```
/// use aoc2022::{Rucksack, Item};
///
/// let rucksack = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp");
/// assert_eq!(rucksack.badge(), Some(Item('p')));
/// assert_eq!(rucksack.score(), Some(16));
///
/// let rucksack = Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
/// assert_eq!(rucksack.badge(), Some(Item('L')));
/// assert_eq!(rucksack.score(), Some(38));
///
/// let rucksack = Rucksack::from("PmmdzqPrVvPwwTWBwg");
/// assert_eq!(rucksack.badge(), Some(Item('P')));
/// assert_eq!(rucksack.score(), Some(42));
///
/// let rucksack = Rucksack::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
/// assert_eq!(rucksack.badge(), Some(Item('v')));
/// assert_eq!(rucksack.score(), Some(22));
///
/// let rucksack = Rucksack::from("ttgJtRGJQctTZtZT");
/// assert_eq!(rucksack.badge(), Some(Item('t')));
/// assert_eq!(rucksack.score(), Some(20));
///
/// let rucksack = Rucksack::from("CrZsJsPPZsGzwwsLwLmpwMDw");
/// assert_eq!(rucksack.badge(), Some(Item('s')));
/// assert_eq!(rucksack.score(), Some(19));
/// ```
#[derive(Debug)]
pub struct Rucksack {
    pub items: Items,
    pub compartments: Group,
}
impl Rucksack {
    pub fn from(input: &str) -> Self {
        let size = input.len() / 2;
        let items = Items::from(input);
        let (l, r) = input.split_at(size);
        let compartments = Group(Vec::from([Items::from(l), Items::from(r)]));
        Self {
            items,
            compartments,
        }
    }

    pub fn badge(&self) -> Option<Item> {
        self.compartments.badge()
    }

    pub fn score(&self) -> Option<u32> {
        Some(self.badge()?.score())
    }
}

#[derive(Debug)]
pub struct Cargo(Vec<Rucksack>);
impl Cargo {
    pub fn load_from(path: &str) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;
        let cargo: Vec<Rucksack> = data.split('\n').map(|r| Rucksack::from(r)).collect();
        Some(Cargo(cargo))
    }

    pub fn individuals_score(&self) -> Option<u32> {
        let scores = self.0.iter().map(|i| i.score());
        let mut score: u32 = 0;
        for s in scores {
            score += s?;
        }
        Some(score)
    }

    pub fn groups_score(&self) -> Option<u32> {
        let scores = self.0.chunks(3).map(|c| {
            let group: Vec<Items> = c.iter().map(|r| r.items.clone()).collect();
            Group(group).score()
        });

        let mut score: u32 = 0;
        for s in scores {
            score += s?;
        }
        Some(score)
    }
}
