use regex;
use std::{collections::HashMap, fs, io, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    ParseInt(ParseIntError),
    ParseRegex(regex::Error),
    InvalidInput,
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

#[derive(Debug, Clone)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Back,
    Front,
}

#[derive(Debug)]
struct Field {
    cell: (usize, usize),
    direction: Direction,
    cells: HashMap<(usize, usize), bool>,
    rows: HashMap<usize, (usize, usize)>,
    cols: HashMap<usize, (usize, usize)>,
}
impl Field {
    pub fn stroll(&mut self, (turn, steps): &(Turn, usize)) {
        self.turn(turn);
        for _ in 0..steps.clone() {
            if !self.step() {
                break;
            }
        }
    }

    fn turn(&mut self, turn: &Turn) {
        self.direction = match (&self.direction, turn) {
            (&Direction::Front, &Turn::Left) => Direction::Left,
            (&Direction::Front, &Turn::Right) => Direction::Right,
            (&Direction::Back, &Turn::Left) => Direction::Right,
            (&Direction::Back, &Turn::Right) => Direction::Left,
            (&Direction::Left, &Turn::Left) => Direction::Back,
            (&Direction::Left, &Turn::Right) => Direction::Front,
            (&Direction::Right, &Turn::Left) => Direction::Front,
            (&Direction::Right, &Turn::Right) => Direction::Back,
        };
    }

    fn step(&mut self) -> bool {
        let cell = match self.direction {
            Direction::Front => self.front(),
            Direction::Back => self.back(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        };
        if self.cells.get(&cell).unwrap() == &true {
            false
        } else {
            self.cell = cell;
            true
        }
    }

    fn front(&self) -> (usize, usize) {
        let mut next = self.cell.clone();
        let &col = self.cols.get(&next.1).unwrap();
        if next.0 == col.0 {
            next.0 = col.1;
        } else {
            next.0 -= 1
        }
        next
    }

    fn back(&self) -> (usize, usize) {
        let mut next = self.cell.clone();
        let &col = self.cols.get(&next.1).unwrap();
        if next.0 == col.1 {
            next.0 = col.0;
        } else {
            next.0 += 1
        }
        next
    }

    fn left(&self) -> (usize, usize) {
        let mut next = self.cell.clone();
        let &row = self.rows.get(&next.0).unwrap();
        if next.1 == row.0 {
            next.1 = row.1;
        } else {
            next.1 -= 1
        }
        next
    }

    fn right(&self) -> (usize, usize) {
        let mut next = self.cell.clone();
        let &row = self.rows.get(&next.0).unwrap();
        if next.1 == row.1 {
            next.1 = row.0;
        } else {
            next.1 += 1
        }
        next
    }
}
impl FromStr for Field {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = Direction::Back;
        let mut cells: HashMap<(usize, usize), bool> = HashMap::new();
        let mut rows: HashMap<usize, (usize, usize)> = HashMap::new();
        let mut cols: HashMap<usize, (usize, usize)> = HashMap::new();
        for (r, line) in s.lines().enumerate() {
            let r = r + 1;
            for (c, item) in line.chars().enumerate() {
                let c = c + 1;
                if item != '.' && item != '#' {
                    continue;
                }
                if let Some(row) = rows.get_mut(&r) {
                    row.1 = c;
                } else {
                    rows.insert(r, (c, c));
                }
                if let Some(col) = cols.get_mut(&c) {
                    col.1 = r;
                } else {
                    cols.insert(c, (r, r));
                }
                cells.insert((r, c), item == '#');
            }
        }
        let cell = (1, rows.get(&1).unwrap().0.clone());

        Ok(Self {
            cell,
            direction,
            cells,
            rows,
            cols,
        })
    }
}

#[derive(Debug, Clone)]
struct Route(Vec<(Turn, usize)>);
impl FromStr for Route {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.replace("L", "_L:");
        s = s.replace("R", "_R:");

        let mut output: Vec<(Turn, usize)> = vec![];
        for step in format!("L:{}", s).split('_') {
            let mut pair = step.split(':');
            let turn = match pair.next().ok_or(Error::InvalidInput)? {
                "L" => Turn::Left,
                "R" => Turn::Right,
                _ => panic!(),
            };
            let steps = pair.next().ok_or(Error::InvalidInput)?.parse()?;
            output.push((turn, steps));
        }
        Ok(Self(output))
    }
}

#[derive(Debug)]
struct MonkeyMap {
    field: Field,
    route: Route,
}
impl FromStr for MonkeyMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.split("\n\n");
        let field = Field::from_str(data.next().ok_or(Error::InvalidInput)?)?;
        let route = Route::from_str(data.next().ok_or(Error::InvalidInput)?)?;
        Ok(Self { field, route })
    }
}
impl MonkeyMap {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        Self::from_str(&fs::read_to_string(path)?)
    }

    pub fn password(&mut self) -> usize {
        for stroll in self.route.0.iter() {
            self.field.stroll(stroll);
        }

        self.field.cell.0 * 1000
            + self.field.cell.1 * 4
            + match self.field.direction {
                Direction::Back => 3,
                Direction::Front => 1,
                Direction::Right => 0,
                Direction::Left => 2,
            }
    }
}

fn main() {
    let mut monkey_map = MonkeyMap::load_from("data/input.txt").unwrap();
    println!("The password is {}", monkey_map.password());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut monkey_map = MonkeyMap::load_from("data/test.txt").unwrap();
        assert_eq!(monkey_map.password(), 6032)
    }
}
