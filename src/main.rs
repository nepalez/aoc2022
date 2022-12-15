use crate::Action::{Math, Yell};
use crate::Error::InvalidOperation;
use crate::Operation::{Add, Divide, Minus, Multiply};
use regex;
use std::collections::{HashMap, VecDeque};
use std::{fs, io, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    ParseInt(ParseIntError),
    ParseRegex(regex::Error),
    InvalidOperation,
    MissedRoot,
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

type Key = String;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Operation {
    Add,
    Multiply,
    Divide,
    Minus,
}
impl Operation {
    pub fn solve(&self, left: i128, right: i128) -> i128 {
        match self {
            Add => left + right,
            Multiply => left * right,
            Divide => left / right,
            Minus => left - right,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Action {
    Yell(i128),
    Math(Operation, Key, Key),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Monkey {
    pub root: bool,
    pub key: Key,
    pub action: Action,
    pub value: Option<i128>,
}
impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = regex::Regex::from_str(r"(.+)[:] (\d+)")?;
        if let Some(cap) = reg.captures(s) {
            if let Some(key) = cap.get(1) {
                let key: Key = key.as_str().into();
                let action = Yell(cap.get(2).ok_or(InvalidOperation)?.as_str().parse()?);
                return Ok(Self {
                    key,
                    action,
                    root: false,
                    value: None,
                });
            }
        }

        let reg = regex::Regex::from_str(r"(.+)[:] (.{4}) (.) (.{4})")?;
        let cap = reg.captures(s).ok_or(InvalidOperation)?;
        let key = cap.get(1).ok_or(InvalidOperation)?.as_str();
        let root = key == "root";
        let key = key.into();
        let left: Key = cap.get(2).ok_or(InvalidOperation)?.as_str().into();
        let right: Key = cap.get(4).ok_or(InvalidOperation)?.as_str().into();
        let operation = match cap.get(3).ok_or(InvalidOperation)?.as_str() {
            "+" => Add,
            "-" => Minus,
            "*" => Multiply,
            "/" => Divide,
            _ => Err(InvalidOperation)?,
        };

        Ok(Self {
            root,
            key,
            action: Math(operation, left, right),
            value: None,
        })
    }
}

#[derive(Debug)]
pub struct Riddle {
    monkeys: HashMap<Key, Monkey>,
    root: Key,
}
impl FromStr for Riddle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkeys = HashMap::new();
        let mut root: Option<Key> = None;
        for line in s.lines() {
            let monkey = Monkey::from_str(line)?;
            if monkey.root {
                root = Some(monkey.key.clone().into());
            }
            monkeys.insert(monkey.key.clone(), monkey);
        }
        if let Some(root) = root {
            Ok(Self { root, monkeys })
        } else {
            Err(Error::MissedRoot)
        }
    }
}
impl Riddle {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        Self::from_str(&fs::read_to_string(path)?)
    }

    pub fn solve(&mut self) -> Option<i128> {
        // build stack
        let mut stack = vec![];
        let mut queue = VecDeque::from([self.root.clone()]);
        while let Some(cursor) = queue.pop_front() {
            let monkey = self.monkeys.get(&cursor)?;
            match &monkey.action {
                Math(_, left, right) => {
                    queue.push_back(right.clone());
                    queue.push_back(left.clone());
                }
                _ => {}
            }
            println!("{:?}", monkey);
            stack.push(cursor.clone());
        }

        // calculate stack
        while let Some(key) = stack.pop() {
            let monkey = self.monkeys.get(&key)?;
            let value = match &monkey.action {
                Yell(value) => value.clone(),
                Math(op, left, right) => {
                    let left = self.monkeys.get(left)?.value?;
                    let right = self.monkeys.get(right)?.value?;
                    op.solve(left, right)
                }
            };
            self.monkeys.get_mut(&key)?.value = Some(value);
        }

        self.monkeys.get(&self.root)?.value
    }
}

fn main() {
    let mut calculator = Riddle::load_from("data/input.txt").unwrap();
    println!("The answer is {}", calculator.solve().unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut calculator = Riddle::load_from("data/test.txt").unwrap();
        assert_eq!(calculator.solve().unwrap(), 152);
    }
}
