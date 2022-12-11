use std::collections::{HashMap, VecDeque};
use std::fs;
use std::slice::Iter;

// List of dividers used by monkeys to test inputs
#[derive(Debug)]
pub(crate) struct Dividers(Vec<usize>);
impl Dividers {
    pub fn iter(&self) -> Iter<'_, usize> {
        self.0.iter()
    }
}
impl From<&str> for Dividers {
    fn from(value: &str) -> Self {
        Self(
            value
                .split('\n')
                .filter_map(|line| Test::from(line))
                .collect(),
        )
    }
}

#[derive(Debug)]
pub(crate) struct Test {}
impl Test {
    pub fn from(input: &str) -> Option<usize> {
        if input.len() < 22 || &input[0..21] != "  Test: divisible by " {
            None
        } else {
            Some(input[21..].parse::<usize>().ok()?)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct OnTrue {}
impl OnTrue {
    pub fn from(input: &str) -> Option<usize> {
        if input.len() < 30 || &input[0..29] != "    If true: throw to monkey " {
            None
        } else {
            Some(input[29..].parse::<usize>().ok()?)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct OnFalse {}
impl OnFalse {
    pub fn from(input: &str) -> Option<usize> {
        if input.len() < 31 || &input[0..30] != "    If false: throw to monkey " {
            None
        } else {
            Some(input[30..].parse::<usize>().ok()?)
        }
    }
}

#[derive(Debug)]
pub(crate) enum Inspection {
    Square,
    Add(usize),
    Multiply(usize),
}
impl Inspection {
    pub fn from(input: &str) -> Option<Self> {
        if input == "  Operation: new = old * old" {
            Some(Self::Square)
        } else if input.len() > 25 && &input[0..25] == "  Operation: new = old * " {
            Some(Self::Multiply(input[25..].parse::<usize>().ok()?))
        } else if input.len() > 25 && &input[0..25] == "  Operation: new = old + " {
            Some(Self::Add(input[25..].parse::<usize>().ok()?))
        } else {
            None
        }
    }
}

// Keep a warning level a a rest of division by all the monkey's dividers
#[derive(Debug, Default)]
pub(crate) struct Level(HashMap<usize, usize>);
impl Level {
    pub fn new(dividers: &Dividers, input: usize) -> Self {
        let mut level: HashMap<usize, usize> = HashMap::default();
        for divider in dividers.iter() {
            level.insert(divider.clone(), input % divider);
        }
        Self(level)
    }

    pub fn is_divided_by(&mut self, divider: &usize) -> Option<bool> {
        Some(self.0.get(&divider)? == &0)
    }

    pub fn modify(self, inspection: &Inspection) -> Self {
        match &inspection {
            Inspection::Add(value) => self.add(value),
            Inspection::Multiply(value) => self.multiply(value),
            _ => self.square(),
        }
    }

    fn add(self, value: &usize) -> Self {
        let mut level: HashMap<usize, usize> = HashMap::default();
        for (divider, rest) in self.0 {
            level.insert(divider, (rest + value) % divider);
        }
        Self(level)
    }

    fn multiply(self, value: &usize) -> Self {
        let mut level: HashMap<usize, usize> = HashMap::default();
        for (&divider, rest) in self.0.iter() {
            level.insert(divider, (rest * value) % divider);
        }
        Self(level)
    }

    fn square(self) -> Self {
        let mut level: HashMap<usize, usize> = HashMap::default();
        for (&divider, rest) in self.0.iter() {
            level.insert(divider, (rest * rest) % divider);
        }
        Self(level)
    }
}

#[derive(Debug, Default)]
pub(crate) struct Levels(VecDeque<Level>);
impl Levels {
    pub fn from(dividers: &Dividers, input: &str) -> Option<Self> {
        if input.len() < 19 || &input[0..18] != "  Starting items: " {
            None
        } else {
            let mut output = Self::default();
            for item in input[18..].split(", ") {
                let value = item.parse::<usize>().ok()?;
                let level = Level::new(dividers, value);
                output.push(level);
            }
            Some(output)
        }
    }

    pub fn push(&mut self, level: Level) {
        self.0.push_back(level);
    }

    pub fn pop(&mut self) -> Option<Level> {
        self.0.pop_front()
    }
}

#[derive(Debug)]
pub(crate) struct Monkey {
    pub levels: Levels,
    pub inspection: Inspection,
    pub divider: usize,
    pub on_true: usize,
    pub on_false: usize,
    pub inspected: usize,
}
impl Monkey {
    pub fn from(dividers: &Dividers, input: &str) -> Option<Self> {
        let mut lines = input.split('\n');

        lines.next(); // skip index
        let queue = Levels::from(dividers.into(), lines.next()?)?;
        let inspection = Inspection::from(lines.next()?)?;
        let test = Test::from(lines.next()?)?;
        let on_true = OnTrue::from(lines.next()?)?;
        let on_false = OnFalse::from(lines.next()?)?;

        Some(Self {
            levels: queue,
            inspection,
            divider: test,
            on_true,
            on_false,
            inspected: 0,
        })
    }

    pub fn play_and_throw(&mut self) -> Option<(usize, Level)> {
        let mut level = self.levels.pop()?.modify(&self.inspection);
        self.inspected += 1;
        if level.is_divided_by(&self.divider)? {
            Some((self.on_true, level))
        } else {
            Some((self.on_false, level))
        }
    }

    pub fn catch(&mut self, level: Level) {
        self.levels.push(level);
    }
}

#[derive(Debug)]
pub struct Monkeys {
    list: Vec<Monkey>,
    size: usize,
}
impl Monkeys {
    pub fn load_from(path: &str) -> Option<Self> {
        let input = fs::read_to_string(path).ok()?;
        Self::from(&input)
    }

    pub fn from(input: &str) -> Option<Self> {
        let mut list = Vec::new();
        let dividers = Dividers::from(input);

        for part in input.split("\n\n") {
            let monkey = Monkey::from(&dividers, part)?;
            list.push(monkey);
        }
        let size = list.len();
        Some(Self { list, size })
    }

    pub fn play_round(&mut self) {
        for index in 0..self.size {
            while let Some((new_index, item)) = { self.list[index].play_and_throw() } {
                self.list[new_index].catch(item);
            }
        }
    }

    pub fn monkey_business(&mut self, rounds: usize) -> usize {
        for _ in 0..rounds {
            self.play_round();
        }
        let mut counters: Vec<usize> = self.list.iter().map(|monkey| monkey.inspected).collect();
        counters.sort_by(|a, b| b.cmp(&a));
        counters[0] * counters[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monkeys() {
        // let mut monkeys = Monkeys::load_from("data/11_test.in").unwrap();
        // for _ in 0..20 {
        //     monkeys.play_round(true);
        // }
        // assert_eq!(monkeys.monkey_business(), 10605);
        //
        let mut monkeys = Monkeys::load_from("data/11_test.in").unwrap();
        assert_eq!(monkeys.monkey_business(10000), 2713310158);
    }
}
