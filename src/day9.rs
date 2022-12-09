use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: usize,
}
impl Motion {
    pub fn from(input: &str) -> Option<Self> {
        let mut data = input.split(' ');
        let direction = match data.next()? {
            "U" => Direction::UP,
            "D" => Direction::DOWN,
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => return None,
        };
        let steps = data.next()?.parse::<u32>().ok()? as usize;
        Some(Self { direction, steps })
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Knot {
    pub x: i32,
    pub y: i32,
}
impl Knot {
    fn follow(&mut self, head: Knot) {
        loop {
            let dx = head.x - self.x;
            let dy = head.y - self.y;
            if dx.abs() < 2 && dy.abs() < 2 {
                break;
            }
            self.x += dx.signum();
            self.y += dy.signum();
        }
    }

    fn follow_tail(&mut self, head: Knot, positions: &mut Positions) {
        loop {
            let dx = head.x - self.x;
            let dy = head.y - self.y;
            if dx.abs() < 2 && dy.abs() < 2 {
                break;
            }
            self.x += dx.signum();
            self.y += dy.signum();
            positions.insert(&self);
        }
    }
}

#[derive(Clone, Debug)]
struct Rope {
    pub knots: Vec<Knot>,
    size: usize,
}
impl Rope {
    pub fn new(size: usize) -> Option<Self> {
        if size > 0 {
            let mut knots = Vec::with_capacity(size);
            for _ in 0..size {
                knots.push(Knot::default())
            }
            Some(Self { knots, size })
        } else {
            None
        }
    }

    pub fn apply(&mut self, motion: &Motion, positions: &mut Positions) {
        for _ in 0..motion.steps {
            self.step(&motion.direction, positions);
        }
    }

    fn tail(&mut self) -> &mut Knot {
        &mut self.knots[self.size - 1]
    }

    fn head(&mut self) -> &mut Knot {
        &mut self.knots[0]
    }

    fn step(&mut self, direction: &Direction, positions: &mut Positions) {
        match direction {
            Direction::UP => self.head().y += 1,
            Direction::DOWN => self.head().y -= 1,
            Direction::RIGHT => self.head().x += 1,
            Direction::LEFT => self.head().x -= 1,
        }
        let mut head = self.head().clone();
        for i in 1..(self.size - 1) {
            let knot = self.knots[i].borrow_mut();
            knot.follow(head);
            head = knot.clone();
        }
        self.tail().follow_tail(head, positions);
    }
}

#[derive(Debug, Default)]
struct Positions(HashSet<Knot>);
impl Positions {
    pub fn new() -> Self {
        Self(HashSet::from([Knot::default()]))
    }

    pub fn insert(&mut self, knot: &Knot) {
        self.0.insert(knot.clone());
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }
}

/// Move the rope
/// ```
/// use std::collections::HashSet;
/// use aoc2022::Motions;
///
/// let motions = Motions::from("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2").unwrap();
/// assert_eq!(motions.count_tail_positions(2), Some(13));
///
/// let motions = Motions::from("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20").unwrap();
/// assert_eq!(motions.count_tail_positions(10), Some(36));
/// ```
#[derive(Debug, Default)]
pub struct Motions(Vec<Motion>);
impl Motions {
    pub fn load_from(path: &str) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;
        Self::from(&data)
    }

    pub fn from(input: &str) -> Option<Self> {
        let size = input.chars().filter(|c| c == &'\n').count();
        let mut motions = Vec::with_capacity(size + 1);
        for line in input.split('\n') {
            motions.push(Motion::from(line)?);
        }
        Some(Self(motions))
    }

    pub fn count_tail_positions(&self, size: usize) -> Option<usize> {
        let mut positions = Positions::new();

        let mut rope = Rope::new(size)?;
        for motion in self.0.iter() {
            rope.apply(motion, &mut positions);
        }

        Some(positions.count())
    }
}
