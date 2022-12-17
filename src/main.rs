use crate::Error::{InvalidKey, InvalidLine};
use regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::{fs, io, num::ParseIntError, str::FromStr};

type Time = usize;
type Release = usize;

#[derive(Debug)]
enum Error {
    ReadInput(io::Error),
    ParseNumber(ParseIntError),
    ParseRegex(regex::Error),
    InvalidLine,
    InvalidKey,
}
impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::ReadInput(error)
    }
}
impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::ParseNumber(error)
    }
}
impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Self::ParseRegex(error)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Key(char, char);
impl FromStr for Key {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c = s.chars();
        Ok(Self(
            c.next().ok_or(InvalidKey)?,
            c.next().ok_or(InvalidKey)?,
        ))
    }
}

#[derive(Debug, Clone)]
struct Valve {
    key: Key,
    release: Release,
    neighbours: HashMap<Key, Time>,
    // used to find next valve
    open: bool,
    visited: bool,
    time_from_start: Option<Time>,
    can_release: Release,
}
impl FromStr for Valve {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let rx = regex::Regex::new(
            r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.+)",
        )?;

        let caps = rx.captures(line).ok_or(InvalidLine)?;
        let key = Key::from_str(caps.get(1).ok_or(InvalidLine)?.as_str())?;
        let score = caps.get(2).ok_or(InvalidLine)?.as_str().parse::<usize>()?;
        let keys = caps
            .get(3)
            .ok_or(InvalidLine)?
            .as_str()
            .split(", ")
            .map(|k| Key::from_str(k));

        let mut valve = Valve {
            key,
            release: score,
            neighbours: HashMap::new(),
            open: false,
            visited: false,
            time_from_start: None,
            can_release: 0,
        };
        for key in keys {
            valve.neighbours.insert(key?, 1);
        }
        Ok(valve)
    }
}

#[derive(Debug, Clone)]
struct Graph {
    list: HashMap<Key, Valve>,
    start: Key,
}
impl FromStr for Graph {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Self {
            list: HashMap::new(),
            start: Key('A', 'A'),
        };
        for line in s.lines() {
            let valve = Valve::from_str(line)?;
            graph.list.insert(valve.key, valve);
        }
        graph.remove_broken_valves();
        graph.estimate_all_times();

        Ok(graph)
    }
}
impl Graph {
    pub fn load_from(path: &str) -> Result<Self, Error> {
        Self::from_str(&fs::read_to_string(path)?)
    }

    fn remove_broken_valves(&mut self) {
        let broken_keys: Vec<Key> = self
            .list
            .iter()
            .filter(|(_, v)| v.release == 0)
            .map(|(&k, _)| k)
            .collect();

        for broken_key in broken_keys.iter() {
            self.remove_valve(broken_key);
        }
    }

    fn remove_valve(&mut self, removed_key: &Key) {
        // prepare list of all paths through the removed key
        let mut list = vec![];
        let removed_value = self.list.get(removed_key).unwrap();

        for (&source_key, source_valve) in self.list.iter() {
            if let Some(&time_from_source) = source_valve.neighbours.get(removed_key) {
                for (&target_key, &time_to_target) in removed_value.neighbours.iter() {
                    if source_key != target_key {
                        list.push((source_key, target_key, time_from_source + time_to_target));
                    }
                }
            }
        }

        // add direct links and remove links to broken key
        for (source_key, target_key, new_time) in list.into_iter() {
            let valve = self.list.get_mut(&source_key).unwrap();
            let old_time = valve
                .neighbours
                .get(&target_key)
                .unwrap_or(&new_time)
                .clone();
            let new_time = old_time.min(new_time.clone());
            valve.neighbours.insert(target_key, new_time);
            valve.neighbours.remove(removed_key);
        }

        // remove broken valve unless it is a start
        if removed_key != &self.start {
            self.list.remove(removed_key);
        }
    }

    fn estimate_all_times(&mut self) {
        let targets: Vec<_> = self.list.iter().map(|(&k, _)| k).collect();
        for key in targets {
            self.estimate_times_from(key);
        }
    }

    fn estimate_times_from(&mut self, start_key: Key) {
        let mut start_valve = self.list.get(&start_key).unwrap().clone();

        // keys of nodes to which we want estimations from start
        let mut visited = HashSet::from([start_key]);
        let mut queue: Vec<Key> = start_valve.neighbours.iter().map(|(&k, _)| k).collect();
        let mut current_key: Key;

        loop {
            // reorder key to pop the closest not visited first
            queue = queue
                .iter()
                .map(|&k| k)
                .filter(|k| !visited.contains(k))
                .collect();
            queue.sort_by(|a, b| {
                let ta = start_valve.neighbours.get(a).unwrap().clone();
                let tb = start_valve.neighbours.get(b).unwrap().clone();
                tb.cmp(&ta)
            });

            // take the closest not visited key from a queue
            match queue.pop() {
                Some(k) => current_key = k,
                None => break,
            }
            let time_from_start = start_valve.neighbours.get(&current_key).unwrap().clone();
            let current_valve = self.list.get(&current_key).unwrap();

            // update its unvisited neighbours with better times
            for (&key, &time) in current_valve.neighbours.iter() {
                if !visited.contains(&key) {
                    let new_time = time_from_start + time;
                    let new_time = start_valve
                        .neighbours
                        .get(&key)
                        .unwrap_or(&new_time)
                        .clone()
                        .min(new_time);
                    start_valve.neighbours.insert(key, new_time);
                    queue.push(key);
                }
            }

            // put it to the visited
            visited.insert(current_key);
        }

        self.list.insert(start_key, start_valve);
    }
}

const TOTAL_LIMIT: usize = 26;
// TODO: just an euristics
//       its better to count it on every step as a max distance from both nodes
const LOCAL_LIMIT: usize = 15;

#[derive(Debug, Clone)]
struct Path<'a> {
    pub released: Release,
    pub paths: (VecDeque<Key>, VecDeque<Key>),
    currents: (&'a Valve, &'a Valve),
    times_left: (Time, Time),
    time_limit: Time,
    visited: HashSet<Key>,
    graph: &'a Graph,
    next_times_left: (Time, Time),
    next_released: Release,
    next_visited: HashSet<Key>,
}
impl<'a> Path<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        let start = graph.list.get(&Key('A', 'A')).unwrap();
        
        Path {
            released: 0,
            paths: (VecDeque::new(), VecDeque::new()),
            currents: (start, start),
            times_left: (TOTAL_LIMIT, TOTAL_LIMIT),
            next_times_left: (TOTAL_LIMIT, TOTAL_LIMIT),
            time_limit: (TOTAL_LIMIT - LOCAL_LIMIT),
            next_released: 0,
            visited: HashSet::from([start.key]),
            next_visited: HashSet::from([start.key]),
            graph,
        }
    }

    pub fn visit(mut self) -> Vec<Self> {
        let mut stopped = (true, true);
        let mut output = vec![];
        for (_, valve) in self.graph.list.iter() {
            if let Some(path) = self.clone().add_first(valve) {
                output.push(path);
                stopped.0 = false;
            }
            if let Some(path) = self.clone().add_second(valve) {
                output.push(path);
                stopped.1 = false;
            }
        }
        if stopped.0 {
            self.times_left.0 = 0;
        }
        if stopped.1 {
            self.times_left.1 = 0;
        }
        if output.is_empty() {
            output.push(self);
        }
        output
    }

    fn add_first(mut self, valve: &'a Valve) -> Option<Self> {
        let key = valve.key;
        if self.times_left.0 <= self.time_limit {
            None
        } else if self.visited.contains(&key) {
            None
        } else if let Some(time) = self.currents.0.neighbours.get(&valve.key) {
            self.visited.insert(key);
            self.times_left.0 = self.times_left.0.saturating_sub(time + 1);
            self.released += valve.release * self.times_left.0;

            if self.paths.0.is_empty() {
                self.next_times_left.0 = self.times_left.0;
                self.next_released += valve.release * self.times_left.0;
                self.next_visited.insert(key);
            }

            self.paths.0.push_back(key);
            self.currents.0 = valve;
            Some(self)
        } else {
            None
        }
    }

    fn add_second(mut self, valve: &'a Valve) -> Option<Self> {
        let key = valve.key;
        if self.times_left.1 <= self.time_limit {
            None
        } else if self.visited.contains(&key) {
            None
        } else if let Some(time) = self.currents.1.neighbours.get(&valve.key) {
            self.visited.insert(key);
            self.times_left.1 = self.times_left.1.saturating_sub(time + 1);
            self.released += valve.release * self.times_left.1;

            if self.paths.1.is_empty() {
                self.next_times_left.1 = self.times_left.1;
                self.next_released += valve.release * self.times_left.1;
                self.next_visited.insert(key);
            }

            self.paths.1.push_back(key);
            self.currents.1 = valve;
            Some(self)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Travel<'a> {
    graph: &'a Graph,
    queue: Vec<Path<'a>>,
    best_path: Path<'a>,
}
impl<'a> Travel<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        let path = Path::new(graph);
        Self {
            graph,
            queue: vec![path.clone()],
            best_path: path,
        }
    }
    
    pub fn calculate(&mut self) {
        loop {
            self.visit_all();

            let mut path = self.best_path.clone();
            if path.paths.0.is_empty() && path.paths.1.is_empty() {
                break;
            }
            
            // remember first valves as new currents
            if let Some(key) = path.paths.0.pop_front() {
                path.currents.0 = self.graph.list.get(&key).unwrap();
            }
            if let Some(key) = path.paths.1.pop_front() {
                path.currents.1 = self.graph.list.get(&key).unwrap();
            }
            path.paths = (VecDeque::new(), VecDeque::new());
            path.visited = path.next_visited.clone();
            path.released = path.next_released;
            path.times_left = path.next_times_left;
            path.time_limit = path.times_left.0.saturating_sub(LOCAL_LIMIT);
            path.time_limit = path.time_limit.min(path.times_left.1.saturating_sub(LOCAL_LIMIT));
            
            self.best_path = path.clone();
            self.queue = vec![path];
        }
    }
    
    pub fn visit_all(&mut self) -> Release {
        while let Some(_) = self.visit() {}
        self.best_path.released
    }

    fn visit(&mut self) -> Option<()> {
        for path in self.queue.pop()?.visit() {
            // when nobody can move
            if path.times_left.0 == 0 && path.times_left.1 == 0 {
                if path.released > self.best_path.released {
                    self.best_path = path;
                }
            } else {
                self.queue.push(path);
            }
        }
        Some(())
    }
}

fn main() {
    let graph = Graph::load_from("data/input.txt").unwrap();
    let mut travel = Travel::new(&graph);
    travel.visit_all();
    println!("Best release is {}", travel.best_path.released);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let graph = Graph::load_from("data/test.txt").unwrap();
        let mut travel = Travel::new(&graph);
        travel.calculate();
        assert_eq!(travel.best_path.released, 1707);
    }
}
