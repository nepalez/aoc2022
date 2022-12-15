use bitmaps::Bitmap;
use std::collections::{HashMap, HashSet};
use std::{fs, io, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum Error {
    ReadInput(io::Error),
    Parse(ParseIntError),
    InvalidCoords,
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

#[derive(Debug, Clone)]
enum Cell {
    Gem(Bitmap<6>),
    Water,
}

#[derive(Debug, Default)]
struct Queue(HashSet<(usize, usize, usize)>);
impl Queue {
    pub fn push(&mut self, item: (usize, usize, usize)) {
        self.0.insert(item);
    }

    pub fn pop(&mut self) -> Option<(usize, usize, usize)> {
        if let Some(&item) = self.0.iter().next() {
            self.0.remove(&item);
            Some(item)
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
struct Pool {
    cells: HashMap<(usize, usize, usize), Cell>,
    queue: Queue,
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
    z_min: usize,
    z_max: usize,
}
impl FromStr for Pool {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut pool = Self::default();
        for line in input.lines() {
            let mut items = line.split(',');
            let x = items.next().ok_or(Error::InvalidCoords)?.parse::<usize>()?;
            let y = items.next().ok_or(Error::InvalidCoords)?.parse::<usize>()?;
            let z = items.next().ok_or(Error::InvalidCoords)?.parse::<usize>()?;
            // increase by 1 to left 0-z for water
            pool.cells
                .insert((x + 1, y + 1, z + 1), Cell::Gem(Bitmap::new()));
            pool.x_max = pool.x_max.max(x + 1);
            pool.y_max = pool.y_max.max(y + 1);
            pool.z_max = pool.z_max.max(z + 1);
        }

        // Extend pool to let water round the gem
        pool.x_max += 1;
        pool.y_max += 1;
        pool.z_max += 1;

        pool.queue.push((pool.x_min, pool.y_min, pool.z_min));
        Ok(pool)
    }
}
impl Pool {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        Self::from_str(&fs::read_to_string(path)?)
    }

    pub fn sides(&mut self) -> usize {
        self.pour_water();
        self.check_gems();

        let mut result = 0;
        for (_, cell) in self.cells.iter() {
            match cell {
                Cell::Gem(gem) => result += gem.len(),
                _ => {}
            }
        }
        result
    }

    // fill with water

    fn pour_water(&mut self) {
        while let Some(coord) = self.queue.pop() {
            if self.cells.contains_key(&coord) {
                continue; // visited
            }
            self.cells.insert(coord, Cell::Water);
            self.enqueue_neighbours(coord);
        }
    }

    fn enqueue_neighbours(&mut self, (x, y, z): (usize, usize, usize)) {
        if x > self.x_min {
            self.enqueue((x - 1, y, z));
        }
        if x < self.x_max {
            self.enqueue((x + 1, y, z));
        }
        if y > self.y_min {
            self.enqueue((x, y - 1, z));
        }
        if y < self.y_max {
            self.enqueue((x, y + 1, z));
        }
        if z > self.z_min {
            self.enqueue((x, y, z - 1));
        }
        if z < self.z_max {
            self.enqueue((x, y, z + 1));
        }
    }

    fn enqueue(&mut self, coord: (usize, usize, usize)) {
        if self.cells.contains_key(&coord) {
            return;
        }
        self.queue.push(coord);
    }

    // check sides

    fn check_gems(&mut self) {
        // prepare list of gems
        let cap = (self.x_max - self.x_min) * (self.y_max - self.y_min) * (self.z_max - self.z_min);
        let mut gems: Vec<(usize, usize, usize)> = Vec::with_capacity(cap);
        for (&coord, cell) in self.cells.iter() {
            match cell {
                Cell::Gem(_) => gems.push(coord),
                _ => {}
            }
        }

        for coord in gems.into_iter() {
            self.check_gem_sides(coord);
        }
    }

    fn check_gem_sides(&mut self, coord: (usize, usize, usize)) {
        let mut gem: Bitmap<6> = Bitmap::new();
        let (x, y, z) = coord;
        if x > self.x_min {
            if let Some(Cell::Water) = self.cells.get(&(x - 1, y, z)) {
                gem.set(0, true);
            }
        }
        if x < self.x_max {
            if let Some(Cell::Water) = self.cells.get(&(x + 1, y, z)) {
                gem.set(1, true);
            }
        }
        if y > self.y_min {
            if let Some(Cell::Water) = self.cells.get(&(x, y - 1, z)) {
                gem.set(2, true);
            }
        }
        if y < self.y_max {
            if let Some(Cell::Water) = self.cells.get(&(x, y + 1, z)) {
                gem.set(3, true);
            }
        }
        if z > self.z_min {
            if let Some(Cell::Water) = self.cells.get(&(x, y, z - 1)) {
                gem.set(4, true);
            }
        }
        if z < self.z_max {
            if let Some(Cell::Water) = self.cells.get(&(x, y, z + 1)) {
                gem.set(5, true);
            }
        }
        self.cells.insert(coord, Cell::Gem(gem));
    }
}

fn main() {
    let mut pool = Pool::load_from("data/input.txt").unwrap();
    println!("The pool has {} sides", pool.sides());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let mut pool = Pool::load_from("data/test.txt").unwrap();
        assert_eq!(pool.sides(), 58);
    }
}
