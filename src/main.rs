use std::collections::{HashMap, HashSet};
use std::{fs, io, num::ParseIntError, str::FromStr};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    ParseInt(ParseIntError),
    EmptyInput,
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

#[derive(Debug, Clone)]
enum Blizzard {
    L,
    R,
    U,
    D,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
    pub time: usize,
}
impl Position {
    pub fn steps(&self) -> Vec<Self> {
        let mut output = vec![
            Self { time: self.time + 1, x: self.x, y: self.y },
            Self { time: self.time + 1, x: self.x + 1, y: self.y },
            Self { time: self.time + 1, x: self.x, y: self.y + 1 },
        ];
        if self.x > 0 {
            output.push(Self {
                time: self.time + 1,
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            output.push(Self {
                time: self.time + 1,
                x: self.x,
                y: self.y - 1,
            });
        }
        output
    }
    
    pub fn coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[derive(Debug, Default)]
struct Area {
    time: usize,
    right: usize,
    bottom: usize,
    blizzards: HashMap<(usize, usize), Vec<Blizzard>>,
}
impl FromStr for Area {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut area = Self::default();
        for (x, line) in s.lines().enumerate() {
            area.right = line.len() - 2;
            area.bottom += 1;
            for (y, c) in line.chars().enumerate() {
                match c {
                    '>' => { area.blizzards.insert((x, y), vec![Blizzard::R]); },
                    '<' => { area.blizzards.insert((x, y), vec![Blizzard::L]); },
                    '^' => { area.blizzards.insert((x, y), vec![Blizzard::U]); },
                    'v' => { area.blizzards.insert((x, y), vec![Blizzard::D]); },
                    _ => {},
                }
            }
        }
        area.bottom -= 2;
        Ok(area)
    }
}
impl From<&Area> for String {
    fn from(area: &Area) -> Self {
        let mut output = format!("minute {}\n", area.time);
        for x in 0..=(area.bottom + 1) {
            for y in 0..=(area.right + 1) {
                if x == 0 && y == 1 || x == area.bottom + 1 && y == area.right {
                    output.push('.');
                } else if x == 0 || y == 0 || x == area.bottom + 1 || y == area.right + 1 {
                    output.push('#');
                } else if let Some(blizzards) = area.blizzards.get(&(x, y)) {
                    if blizzards.len() > 1 {
                        output.push('@');
                    } else if let Some(blizzard) = blizzards.iter().next() {
                        match blizzard {
                            &Blizzard::R => output.push('>'),
                            &Blizzard::L => output.push('<'),
                            &Blizzard::U => output.push('^'),
                            &Blizzard::D => output.push('v'),
                        }
                    }
                } else {
                    output.push('.');
                }
            }
            output.push('\n');
        }
        output
    }
}
impl Display for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from(self))
    }
}
impl Area {
    /// The next state of the area
    pub fn next(&mut self) {
        let mut blizzards = HashMap::with_capacity(self.blizzards.len());
        for (&(x, y), list) in self.blizzards.iter() {
            for item in list.iter() {
                let coords = match item {
                    &Blizzard::L => {
                        if y == 1 {
                            (x, self.right)
                        } else {
                            (x, y - 1)
                        }
                    }
                    &Blizzard::R => {
                        if y == self.right {
                            (x, 1)
                        } else {
                            (x, y + 1)
                        }
                    }
                    &Blizzard::U => {
                        if x == 1 {
                            (self.bottom, y)
                        } else {
                            (x - 1, y)
                        }
                    }
                    &Blizzard::D => {
                        if x == self.bottom {
                            (1, y)
                        } else {
                            (x + 1, y)
                        }
                    }
                };
                let mut new_list = blizzards.get(&coords).unwrap_or(&vec![]).clone();
                new_list.push(item.clone());
                blizzards.insert(coords, new_list);
            }
        }
        self.blizzards = blizzards;
        self.time += 1;
    }

    /// List of possible steps from a position
    pub fn steps(&self, pos: &Position) -> Vec<Position> {
        if pos.time + 1 != self.time {
            panic!()
        }
        let mut steps = vec![];
        for step in pos.steps() {
            if step.x == 0 && step.y == 1 || step.x == self.bottom + 1 && step.y == self.right {
                steps.push(step);
            } else if step.x >= 1
                && step.x <= self.bottom
                && step.y >= 1
                && step.y <= self.right
                && !self.blizzards.contains_key(&step.coords())
            {
                steps.push(step);
            }
        }
        steps
    }
}

#[derive(Debug, Default)]
struct Route {
    area: Area,
    queue: HashSet<Position>,
}
impl FromStr for Route {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut area = Area::from_str(s)?;
        let queue = HashSet::from([Position { x: 0, y: 1, time: 0 }]);
        Ok(Self { area, queue })
    }
}
impl Route {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        Self::from_str(&fs::read_to_string(path)?)
    }
    
    pub fn calculate(&mut self) -> Option<usize> {
        let mut prev = HashSet::from([Position { time: 0, x: 0, y: 1 }]);
        loop {
            self.area.next();
            let mut next = HashSet::new();
            for pos in prev.iter() {
                if pos.x > self.area.bottom {
                    return Some(pos.time);
                }
                for step in self.area.steps(&pos) {
                    next.insert(step);
                }
            }
            if next.is_empty() {
                return None;
            }
            prev = next;
        }
    }
}

fn main() {
    let mut route = Route::load_from("data/input.txt").unwrap();
    println!("The exit has been reached on minute {:?}", route.calculate());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut route = Route::load_from("data/test2.txt").unwrap();
        assert_eq!(route.calculate().unwrap(), 18);
    }
}
