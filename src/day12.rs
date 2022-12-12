use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::vec::IntoIter;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position(usize, usize);
impl Position {
    pub fn neighbours(&self) -> IntoIter<Position> {
        let mut output: Vec<Self> = Vec::with_capacity(4);
        output.push(Position(self.0 + 1, self.1));
        output.push(Position(self.0, self.1 + 1));
        if self.0 > 0 {
            output.push(Self(self.0 - 1, self.1));
        }
        if self.1 > 0 {
            output.push(Self(self.0, self.1 - 1));
        }
        output.into_iter()
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Node {
    height: usize,
    distance: Option<usize>,
    visited: bool,
}
impl TryFrom<char> for Node {
    type Error = String;

    fn try_from(mut value: char) -> Result<Self, Self::Error> {
        let mut node = Self::default();
        if value == 'S' {
            value = 'a';
        } else if value == 'E' {
            value = 'z';
        }

        if let Some(height) = ('a'..='z').position(|c| c == value) {
            node.height = height;
            Ok(node)
        } else {
            Err(format!("Unexpected symbol: {:?}", value))
        }
    }
}
impl Node {
    pub fn update_distance(&mut self, new_distance: usize) {
        if let Some(old_distance) = self.distance {
            if old_distance > new_distance {
                self.distance = Some(new_distance);
            }
        } else {
            self.distance = Some(new_distance);
        }
    }
}

#[derive(Debug, Default)]
pub struct Grid {
    nodes: HashMap<Position, Node>,
    queue: Vec<Position>,
    finish: Option<Position>,
    start: Option<Position>,
}
impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = s.len();
        let mut grid = Self {
            nodes: HashMap::with_capacity(size),
            queue: Vec::with_capacity(f32::sqrt(size as f32) as usize),
            finish: None,
            start: None,
        };

        for (row, line) in s.split('\n').enumerate() {
            for (col, char) in line.chars().enumerate() {
                let pos = Position(row, col);
                if char == 'S' {
                    grid.start = Some(pos);
                } else if char == 'E' {
                    grid.finish = Some(pos);
                }
                grid.nodes.insert(pos, Node::try_from(char)?);
            }
        }

        if grid.start == None {
            Err(format!("Cannot recognize start"))
        } else if grid.finish == None {
            Err(format!("Cannot recogrnize finish"))
        } else {
            grid.count_distances();
            Ok(grid)
        }
    }
}
impl Grid {
    pub fn load_from(path: &str) -> Option<Self> {
        Self::from_str(&fs::read_to_string(path).ok()?).ok()
    }

    // count distance starting from my position
    pub fn my_distance(&self) -> Option<usize> {
        self.nodes.get(&self.start?)?.distance
    }

    // count shortest distance from bottom to top
    pub fn min_distance(&self) -> Option<usize> {
        let mut output = None;
        for (_, node) in self.nodes.iter().filter(|(_, &n)| n.height == 0) {
            if node.height == 0 {
                if let Some(new_distance) = node.distance {
                    if let Some(old_distance) = output {
                        if new_distance < old_distance {
                            output = Some(new_distance);
                        }
                    } else {
                        output = Some(new_distance);
                    }
                }
            }
        }
        output
    }

    // count distance starting from given position
    fn count_distances(&mut self) {
        self.push_to_queue(self.finish.unwrap(), 0);
        while let Some(_) = self.visit_next() {}
    }

    // visit next node in a queue until the queue is empty
    fn visit_next(&mut self) -> Option<()> {
        let (pos, node) = self.pop_from_queue()?;
        for neighbor in self.neighbours(pos, node) {
            self.push_to_queue(neighbor, node.distance.unwrap() + 1);
        }
        Some(())
    }

    // push unvisited node to queue after updating its distance
    fn push_to_queue(&mut self, position: Position, distance: usize) {
        if let Some(node) = self.nodes.get_mut(&position) {
            if !node.visited {
                node.update_distance(distance);
                self.queue.push(position);
            }
        }
    }

    // pop node from a queue (to check its neighbours)
    fn pop_from_queue(&mut self) -> Option<(Position, Node)> {
        self.sort_queue();
        while let Some(pos) = self.queue.pop() {
            let node = self.nodes.get_mut(&pos).unwrap();
            if !node.visited {
                node.visited = true;
                return Some((pos, node.clone()));
            }
        }
        None
    }

    // filter queue by descending distance of its nodes
    fn sort_queue(&mut self) {
        self.queue.sort_by(|a, b| {
            let da = self.nodes.get(a).unwrap().distance.unwrap();
            let db = self.nodes.get(b).unwrap().distance.unwrap();
            db.cmp(&da)
        });
    }

    // list of the node's potential neighbors
    fn neighbours(&self, position: Position, node: Node) -> Vec<Position> {
        position
            .neighbours()
            .filter(|pos| {
                if let Some(neighbor) = self.nodes.get(pos) {
                    node.height < 2 + neighbor.height
                } else {
                    false
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid() {
        let grid = Grid::load_from("data/12_test.in").unwrap();
        assert_eq!(grid.my_distance(), Some(31));
        assert_eq!(grid.min_distance(), Some(29));
    }
}
