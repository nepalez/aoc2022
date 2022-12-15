use crate::Action::{Math, Skip, Yell};
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
    Skip(Key),
    Yell(i128),
    Math(Operation, Key, Key),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    pub key: Key,
    pub action: Action,
    pub value: Option<i128>,
}
impl FromStr for Node {
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
                    value: None,
                });
            }
        }

        let reg = regex::Regex::from_str(r"(.+)[:] (.{4}) (.) (.{4})")?;
        let cap = reg.captures(s).ok_or(InvalidOperation)?;
        let key = cap.get(1).ok_or(InvalidOperation)?.as_str();
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
            key,
            action: Math(operation, left, right),
            value: None,
        })
    }
}

#[derive(Debug)]
pub struct Solver {
    nodes: HashMap<Key, Node>,
}
impl FromStr for Solver {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashMap::new();
        for line in s.lines() {
            let node = Node::from_str(line)?;
            nodes.insert(node.key.clone(), node);
        }
        Ok(Self { nodes })
    }
}
impl Solver {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        Self::from_str(&fs::read_to_string(path)?)
    }

    pub fn solve(&mut self) -> Option<i128> {
        // build the tree node => parent
        let mut parents: HashMap<Key, Key> = HashMap::new();
        for (key, node) in self.nodes.iter() {
            match &node.action {
                Math(_, left, right) => {
                    parents.insert(left.clone(), key.clone());
                    parents.insert(right.clone(), key.clone());
                }
                _ => {}
            }
        }

        // rebuild the human node
        let mut prev = "humn";
        let mut curr = parents.get(prev)?;
        self.nodes.get_mut(prev)?.action = Skip(curr.clone());

        // go up and rebuild parent nodes from root to humn
        let mut next: &Key;
        while curr != "root" {
            next = parents.get(curr)?;
            let action = if let Math(op, left, right) = &self.nodes.get(curr)?.action {
                match op {
                    Add => {
                        if left == prev {
                            Math(Minus, next.clone(), right.clone())
                        } else {
                            Math(Minus, next.clone(), left.clone())
                        }
                    }
                    Minus => {
                        if left == prev {
                            Math(Add, next.clone(), right.clone())
                        } else {
                            Math(Minus, left.clone(), next.clone())
                        }
                    }
                    Multiply => {
                        if left == prev {
                            Math(Divide, next.clone(), right.clone())
                        } else {
                            Math(Divide, next.clone(), left.clone())
                        }
                    }
                    Divide => {
                        if left == prev {
                            Math(Multiply, next.clone(), right.clone())
                        } else {
                            Math(Divide, left.clone(), next.clone())
                        }
                    }
                }
            } else {
                panic!()
            };
            self.nodes.insert(
                curr.into(),
                Node {
                    key: curr.clone(),
                    action,
                    value: None,
                },
            );
            (prev, curr) = (curr, next);
        }

        // rebuild the root node
        let action = if let Math(_, left, right) = &self.nodes.get(curr)?.action {
            if left == prev {
                Skip(right.clone())
            } else {
                Skip(left.clone())
            }
        } else {
            panic!()
        };
        self.nodes.insert(
            curr.into(),
            Node {
                key: curr.clone(),
                action,
                value: None,
            },
        );

        // build stack
        let mut stack = vec![];
        let mut queue = VecDeque::from([String::from("humn")]);
        while let Some(cursor) = queue.pop_front() {
            let node = self.nodes.get(&cursor)?;
            match &node.action {
                Math(_, left, right) => {
                    queue.push_back(right.clone());
                    queue.push_back(left.clone());
                }
                Skip(key) => {
                    queue.push_back(key.clone());
                }
                _ => {}
            }
            stack.push(cursor.clone());
        }

        // calculate stack
        while let Some(key) = stack.pop() {
            let node = self.nodes.get(&key)?;
            let value = match &node.action {
                Yell(value) => value.clone(),
                Skip(child) => self.nodes.get(child)?.value?,
                Math(op, left, right) => {
                    let left = self.nodes.get(left)?.value?;
                    let right = self.nodes.get(right)?.value?;
                    op.solve(left, right)
                }
            };
            self.nodes.get_mut(&key)?.value = Some(value);
        }

        self.nodes.get("humn")?.value
    }
}

fn main() {
    let mut solver = Solver::load_from("data/input.txt").unwrap();
    println!("The answer is {}", solver.solve().unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut solver = Solver::load_from("data/test.txt").unwrap();
        assert_eq!(solver.solve().unwrap(), 301);
    }
}
