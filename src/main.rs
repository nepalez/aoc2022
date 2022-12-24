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

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Move((usize, usize), Direction);

#[derive(Debug)]
struct Field {
    cell: (usize, usize),
    direction: Direction,
    cells: HashMap<(usize, usize), bool>,
    rows: HashMap<usize, (usize, usize)>,
    cols: HashMap<usize, (usize, usize)>,
    wraps: HashMap<Move, Move>,
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
            (&Direction::Up, &Turn::Left) => Direction::Left,
            (&Direction::Up, &Turn::Right) => Direction::Right,
            (&Direction::Down, &Turn::Left) => Direction::Right,
            (&Direction::Down, &Turn::Right) => Direction::Left,
            (&Direction::Left, &Turn::Left) => Direction::Down,
            (&Direction::Left, &Turn::Right) => Direction::Up,
            (&Direction::Right, &Turn::Left) => Direction::Up,
            (&Direction::Right, &Turn::Right) => Direction::Down,
        };
    }

    fn step(&mut self) -> bool {
        let mut cell = self.cell.clone();
        let mut direction = self.direction.clone();
        if let Some(Move(wrap, dir)) = self.wraps.get(&Move(cell, direction.clone())) {
            (cell, direction) = (wrap.clone(), dir.clone());
        } else {
            match self.direction {
                Direction::Up => cell.0 -= 1,
                Direction::Down => cell.0 += 1,
                Direction::Left => cell.1 -= 1,
                Direction::Right => cell.1 += 1,
            }
        }
        if self.cells.get(&cell).unwrap() == &true {
            false
        } else {
            self.cell = cell;
            self.direction = direction.clone();
            true
        }
    }

    fn populate_wraps(&mut self) {
        // for i in 1..=4 {
        //     self.wraps.insert(
        //         Move((1, 8 + i), Direction::Up),
        //         Move((5, 5 - i), Direction::Down),
        //     );
        //     self.wraps.insert(
        //         Move((5, i), Direction::Up),
        //         Move((1, 13 - i), Direction::Down),
        //     );
        //     self.wraps.insert(
        //         Move((5, 4 + i), Direction::Up),
        //         Move((i, 9), Direction::Right),
        //     );
        //     self.wraps.insert(
        //         Move((i, 9), Direction::Left),
        //         Move((5, 4 + i), Direction::Down),
        //     );
        //     self.wraps.insert(
        //         Move((i, 12), Direction::Right),
        //         Move((13 - i, 16), Direction::Left),
        //     );
        //     self.wraps.insert(
        //         Move((8 + i, 16), Direction::Right),
        //         Move((5 - i, 12), Direction::Left),
        //     );
        //     self.wraps.insert(
        //         Move((4 + i, 12), Direction::Right),
        //         Move((9, 17 - i), Direction::Down),
        //     );
        //     self.wraps.insert(
        //         Move((9, 12 + i), Direction::Up),
        //         Move((9 - i, 12), Direction::Left),
        //     );
        //     self.wraps.insert(
        //         Move((4 + i, 1), Direction::Left),
        //         Move((12, 17 - i), Direction::Up),
        //     );
        //     self.wraps.insert(
        //         Move((12, 12 + i), Direction::Down),
        //         Move((9 - i, 1), Direction::Right),
        //     );
        //     self.wraps.insert(
        //         Move((8, i), Direction::Down),
        //         Move((12, 13 - i), Direction::Up),
        //     );
        //     self.wraps.insert(
        //         Move((12, 8 + i), Direction::Down),
        //         Move((8, 5 - i), Direction::Up),
        //     );
        //     self.wraps.insert(
        //         Move((8, 4 + i), Direction::Down),
        //         Move((13 - i, 9), Direction::Right),
        //     );
        //     self.wraps.insert(
        //         Move((8 + i, 9), Direction::Left),
        //         Move((8, 9 - i), Direction::Up),
        //     );
        // }

        for i in 1..=50 {
            self.wraps.insert(
                Move((1, 50 + i), Direction::Up),
                Move((150 + i, 1), Direction::Right),
            );
            self.wraps.insert(
                Move((150 + i, 1), Direction::Left),
                Move((1, 50 + i), Direction::Down),
            );
            self.wraps.insert(
                Move((1, 100 + i), Direction::Up),
                Move((200, i), Direction::Up),
            );
            self.wraps.insert(
                Move((200, i), Direction::Down),
                Move((1, 100 + i), Direction::Down),
            );
            self.wraps.insert(
                Move((i, 150), Direction::Right),
                Move((151 - i, 100), Direction::Left),
            );
            self.wraps.insert(
                Move((100 + i, 100), Direction::Right),
                Move((51 - i, 150), Direction::Left),
            );
            self.wraps.insert(
                Move((50, 100 + i), Direction::Down),
                Move((50 + i, 100), Direction::Left),
            );
            self.wraps.insert(
                Move((50 + i, 100), Direction::Right),
                Move((50, 100 + i), Direction::Up),
            );
            self.wraps.insert(
                Move((150, 50 + i), Direction::Down),
                Move((150 + i, 50), Direction::Left),
            );
            self.wraps.insert(
                Move((150 + i, 50), Direction::Right),
                Move((150, 50 + i), Direction::Up),
            );
            self.wraps.insert(
                Move((i, 51), Direction::Left),
                Move((151 - i, 1), Direction::Right),
            );
            self.wraps.insert(
                Move((100 + i, 1), Direction::Left),
                Move((51 - i, 51), Direction::Right),
            );
            self.wraps.insert(
                Move((50 + i, 51), Direction::Left),
                Move((101, i), Direction::Down),
            );
            self.wraps.insert(
                Move((101, i), Direction::Up),
                Move((50 + i, 51), Direction::Right),
            );
        }
    }
}
impl FromStr for Field {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

        let mut field = Self {
            cells,
            rows,
            cols,
            cell,
            wraps: HashMap::new(),
            direction: Direction::Down,
        };
        field.populate_wraps();
        Ok(field)
    }
}

#[derive(Debug)]
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
                Direction::Down => 1,
                Direction::Up => 3,
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
        assert_eq!(monkey_map.password(), 5031)
    }
}
