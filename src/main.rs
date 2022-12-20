use std::{fs, io, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    Parse(ParseIntError),
}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::ReadInput(error)
    }
}
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::Parse(error)
    }
}

#[derive(Debug, Default)]
pub struct List {
    items: Vec<(usize, i128)>,
    size: i128,
}
impl List {
    pub fn shift(&mut self, index: usize, number: i128) {
        let old_index = self.find(index, number);
        let mut new_index = (old_index as i128 + number) % (self.size - 1);
        if new_index <= 0 {
            new_index += self.size - 1;
        }
        let new_index = new_index as usize;
        self.items.remove(old_index);
        self.items.insert(new_index, (index, number));
    }

    pub fn find(&self, index: usize, number: i128) -> usize {
        self.items
            .iter()
            .position(|&(i, n)| i == index && n == number)
            .unwrap()
    }

    pub fn find_number(&self, number: i128) -> usize {
        self.items.iter().position(|&(_, n)| n == number).unwrap()
    }

    pub fn get(&self, position: usize) -> (usize, i128) {
        self.items
            .get(position % (self.size as usize))
            .unwrap()
            .clone()
    }
}
impl From<&Vec<i128>> for List {
    fn from(input: &Vec<i128>) -> Self {
        let mut items = vec![];
        for (index, &number) in input.iter().enumerate() {
            items.push((index, number));
        }
        Self {
            items,
            size: input.len() as i128,
        }
    }
}

#[derive(Debug)]
pub struct Cipher {
    source: Vec<i128>,
    list: List,
}
impl FromStr for Cipher {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut source = vec![];
        for line in s.lines() {
            let number = line.parse::<i128>()? * 811589153;
            source.push(number);
        }
        let list = List::from(&source);
        Ok(Self { source, list })
    }
}
impl Cipher {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        Self::from_str(&fs::read_to_string(path)?)
    }
    
    pub fn decrypt(&mut self) -> i128 {
        for _ in 0..10 {
            for (i, &n) in self.source.iter().enumerate() {
                self.list.shift(i, n);
            }
        }

        let init = self.list.find_number(0);
        let mut result = 0;
        for shift in vec![1000, 2000, 3000] {
            result += self.list.get(init + shift).1;
        }

        result
    }
}

fn main() {
    let mut cipher = Cipher::load_from("data/input.txt").unwrap();
    println!("The answer is {}", cipher.decrypt());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut cipher = Cipher::load_from("data/test.txt").unwrap();
        assert_eq!(cipher.decrypt(), 1623178306);
    }
}
