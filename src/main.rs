use crate::Material::{Clay, Geode, Obsidian, Ore};
use regex;
use std::cmp::Ordering;
use std::{collections::HashMap, fs, io, num::ParseIntError, str::FromStr};

const LIMIT: usize = 24;

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    ParseInt(ParseIntError),
    ParseRegex(regex::Error),
    UnexpectedMaterial(String),
    BlueprintError(usize),
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

#[derive(Debug, Clone, Copy, Ord, Eq, PartialEq, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
impl FromStr for Material {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(Ore),
            "obsidian" => Ok(Obsidian),
            "clay" => Ok(Clay),
            "geode" => Ok(Geode),
            _ => Err(Error::UnexpectedMaterial(s.into())),
        }
    }
}
impl PartialOrd for Material {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (&Ore, &Ore) => Some(Ordering::Equal),
            (&Ore, _) => Some(Ordering::Less),
            (_, &Ore) => Some(Ordering::Greater),
            (&Clay, &Clay) => Some(Ordering::Equal),
            (&Clay, _) => Some(Ordering::Less),
            (_, &Clay) => Some(Ordering::Greater),
            (&Obsidian, &Obsidian) => Some(Ordering::Equal),
            (&Obsidian, _) => Some(Ordering::Less),
            (_, &Obsidian) => Some(Ordering::Greater),
            (&Geode, &Geode) => Some(Ordering::Equal),
        }
    }
}

/// The state of the process
#[derive(Debug, Clone)]
struct Process<'a> {
    minutes: usize,
    stock: HashMap<Material, usize>,
    robots: HashMap<Material, usize>,
    blueprint: &'a Blueprint,
    history: Vec<Material>,
}
impl<'a> From<&'a Blueprint> for Process<'a> {
    fn from(blueprint: &'a Blueprint) -> Self {
        Self {
            stock: HashMap::from([(Ore, 0), (Clay, 0), (Obsidian, 0), (Geode, 0)]),
            robots: HashMap::from([(Ore, 1), (Clay, 0), (Obsidian, 0), (Geode, 0)]),
            blueprint: &blueprint,
            minutes: 0,
            history: vec![],
        }
    }
}
impl<'a> Process<'a> {
    fn eventual_geodes(&self) -> usize {
        self.current_stock(&Geode) + LIMIT.saturating_sub(self.minutes) * self.current_robots(&Geode)
    }
    
    fn next_versions(&self) -> Vec<Self> {
        let mut output = vec![];
        
        // prepare list of variants
        let mut states = vec![];
        for m in vec![Ore, Clay, Obsidian, Geode] {
            let time = self.time_to_build(&m);
            if time == usize::MAX {
                break
            }
            let mut state = self.clone();
            state.build_robot(&m);
            
            // if the branch affect the result
            if state.minutes >= LIMIT {
                continue
            }

            states.push(state);
        }
        if states.is_empty() {
            return vec![];
        }

        // get the senior out
        output.push(states.pop().unwrap());
        
        // order versions by cycles
        states.sort_by(|a, b| {
            let a = a.cycle();
            let b = b.cycle();

            let result = if a.1 == i32::MAX || b.1 == i32::MAX {
                b.0.cmp(&a.0)
            } else {
                b.1.cmp(&a.1)
            };
            if let Ordering::Equal = result {
                b.0.cmp(&a.0)
            } else {
                result
            }
        });

        // get the best option if it exists
        if let Some(mut first) = states.pop() {
            // try mixing the second option with the best one to decide if they can be merged
            if let Some(second) = states.pop() {
                let mut mixed = self.clone();
                mixed.build_sequence(&vec![second.history.clone().pop().unwrap(), first.history.clone().pop().unwrap()]);
                if mixed.minutes <= first.minutes {
                    first = second;
                }
            }

            // try mixing the selected version with the geode-building to decide if they can be merged
            if let Some(geode) = output.pop() {
                let mut mixed = self.clone();
                mixed.build_sequence(&vec![first.history.clone().pop().unwrap(), geode.history.clone().pop().unwrap()]);
                if mixed.minutes > first.minutes {
                    output.push(geode);
                }
            }

            output.push(first);
        }

        // it can be empty
        output
    }
    
    // Build a sequence of robots
    fn build_sequence(&mut self, robots: &Vec<Material>) {
        for robot in robots.iter() {
            self.build_robot(robot);
        }
    }

    // Add a new robot and return its blocking material
    fn build_robot(&mut self, robot: &Material) {
        let time = self.time_to_build(robot);
        self.minutes += time;

        for material in vec![Ore, Clay, Obsidian, Geode] {
            let stock = self.current_stock(&material);
            let robots = self.current_robots(&material);
            let consumption = self.consumption(&material, robot);
            
            self.stock.insert(material, stock + robots * time - consumption);
        }

        self.add_robot(robot.clone());
        self.history.push(robot.clone());
    }

    // Time to build the geode robot
    fn cycle(&self) -> (i32, i32) {
        if self.current_robots(&Clay) == 0 {
            return (i32::MAX, i32::MAX);
        }

        let mut minor = self.consumption(&Ore, &Clay) as i32 / self.current_robots(&Clay) as i32;
        minor = minor.max(self.consumption(&Ore, &Obsidian) as i32 / self.current_robots(&Ore) as i32);
        minor = minor.max(self.consumption(&Clay, &Obsidian) as i32 / self.current_robots(&Clay) as i32);
       if self.current_robots(&Obsidian) == 0 {
            return (minor, i32::MAX)
        }
 
        let mut major = minor.max(self.consumption(&Ore, &Geode) as i32 / self.current_robots(&Ore) as i32);
        major = major.max(self.consumption(&Obsidian, &Geode) as i32 / self.current_robots(&Obsidian) as i32);
        (minor, major)
    }

    fn add_robot(&mut self, material: Material) {
        let robots = self.current_robots(&material);
        self.robots.insert(material, robots + 1);
    }

    fn time_to_build(&self, target: &Material) -> usize {
        let mut time = 1;
        for source in vec![Ore, Clay, Obsidian] {
            let deficit = self.deficit(&source, target) as f32;
            if deficit == 0.0 {
                continue
            }
            let robots = self.current_robots(&source) as f32;
            if robots == 0.0 {
                return usize::MAX
            }
            time = time.max((deficit / robots).ceil() as usize + 1);
        }
        time
    }

    fn deficit(&self, source: &Material, target: &Material) -> usize {
        self.consumption(source, target)
            .saturating_sub(self.current_stock(source))
    }

    fn current_stock(&self, material: &Material) -> usize {
        self.stock.get(material).unwrap().clone()
    }

    fn current_robots(&self, material: &Material) -> usize {
        self.robots.get(material).unwrap().clone()
    }

    fn consumption(&self, source: &Material, target: &Material) -> usize {
        self.blueprint.consumption(source, target)
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: usize,
    factory: HashMap<Material, HashMap<Material, usize>>,
}
impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rx = regex::Regex::new(r"Blueprint (\d+)")?;
        let id = rx
            .captures(s)
            .ok_or(Error::BlueprintError(7))?
            .get(1)
            .ok_or(Error::BlueprintError(8))?
            .as_str()
            .parse()?;

        let rx = regex::Regex::from_str("Each (.+) robot costs (.+)")?;
        let mut factory = HashMap::new();
        for line in s.split('.') {
            if line.is_empty() {
                continue;
            }
            let cap = rx.captures(line).ok_or(Error::BlueprintError(1))?;
            let robot = Material::from_str(cap.get(1).ok_or(Error::BlueprintError(2))?.as_str())?;
            let mut stock = HashMap::new();
            for ln in cap
                .get(2)
                .ok_or(Error::BlueprintError(3))?
                .as_str()
                .split(" and ")
            {
                let mut parts = ln.split(' ').into_iter();
                let count = parts
                    .next()
                    .ok_or(Error::BlueprintError(4))?
                    .parse::<usize>()?;
                let material = Material::from_str(parts.next().ok_or(Error::BlueprintError(5))?)?;
                stock.insert(material, count);
            }
            factory.insert(robot, stock);
        }

        Ok(Self { id, factory })
    }
}
impl Blueprint {
    pub fn quality(&self) -> usize {
        let mut result = Process::from(self);
        let mut queue = vec![result.clone()];
        while let Some(state) = queue.pop() {
            let versions = state.next_versions();
            if versions.is_empty() {
                if state.eventual_geodes() > result.eventual_geodes() {
                    result = state;
                }
            } else {
                for version in versions {
                    queue.push(version)
                }
            }
        }
        result.eventual_geodes() * self.id
    }

    pub fn consumption(&self, source: &Material, target: &Material) -> usize {
        self.factory
            .get(&target)
            .unwrap()
            .get(&source)
            .unwrap_or(&0)
            .clone()
    }
}

/// The list of Blueprints
#[derive(Debug)]
struct Factory(Vec<Blueprint>);
impl FromStr for Factory {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blueprints = vec![];
        for line in s.lines() {
            blueprints.push(Blueprint::from_str(line)?)
        }
        Ok(Self(blueprints))
    }
}
impl Factory {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        Self::from_str(&fs::read_to_string(path)?)
    }

    pub fn quality(&self) -> usize {
        self.0.iter().map(|blueprint| blueprint.quality()).sum()
    }
}

fn main() {
    let factory = Factory::load_from("data/input.txt").unwrap();
    println!("Quality: {}", factory.quality());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut factory = Factory::load_from("data/input.txt").unwrap();
        assert_eq!(factory.quality(), 33);
    }
}
