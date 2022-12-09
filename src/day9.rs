use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::fs;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Knot {
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
pub struct Rope {
    pub knots: Vec<Knot>,
    size: usize,
}
impl Rope {
    pub fn new(size: usize) -> Option<Self> {
        if size > 0 {
            let knot = Knot::default();
            let mut knots = Vec::with_capacity(size);
            for _ in 0..size {
                knots.push(knot.clone())
            }
            Some(Self { knots, size })
        } else {
            None
        }
    }

    pub fn motion(&mut self, input: &str, positions: &mut Positions) -> Option<()> {
        let mut data = input.split(' ');
        let direction = data.next()?;
        let steps: u32 = data.next()?.parse().ok()?;
        for _ in 0..steps {
            self.shift(direction, positions)?;
        }
        Some(())
    }

    fn tail(&mut self) -> &mut Knot {
        &mut self.knots[self.size - 1]
    }

    fn head(&mut self) -> &mut Knot {
        &mut self.knots[0]
    }

    fn shift(&mut self, direction: &str, positions: &mut Positions) -> Option<()> {
        match direction {
            "U" => self.head().y += 1,
            "D" => self.head().y -= 1,
            "R" => self.head().x += 1,
            "L" => self.head().x -= 1,
            _ => return None,
        }
        let mut head = self.head().clone();
        for i in 1..(self.size - 1) {
            let knot = self.knots[i].borrow_mut();
            knot.follow(head);
            head = knot.clone();
        }
        self.tail().follow_tail(head, positions);
        Some(())
    }
}

/// Collection of all ropes and knots visited
#[derive(Debug, Default)]
pub struct Positions(HashSet<Knot>);
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
/// use aoc2022::{Motions, Rope, Knot};
///
/// let motions = Motions::from("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2").unwrap();
/// let tail_positions = motions.tail_positions(2).unwrap();
/// assert_eq!(tail_positions.count(), 13);
///
/// let motions = Motions::from("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20").unwrap();
/// let tail_positions = motions.tail_positions(10).unwrap();
/// assert_eq!(tail_positions.count(), 36);
/// ```
#[derive(Debug, Default)]
pub struct Motions(Vec<String>);
impl Motions {
    pub fn load_from(path: &str) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;
        Self::from(&data)
    }

    pub fn from(input: &str) -> Option<Self> {
        let mut motions = Vec::new();
        for motion in input.split('\n') {
            motions.push(motion.into());
        }
        Some(Self(motions))
    }

    pub fn tail_positions(&self, size: usize) -> Option<Positions> {
        let mut positions = Positions::new();

        let mut rope = Rope::new(size)?;
        for motion in self.0.iter() {
            rope.motion(motion, &mut positions)?;
        }

        Some(positions)
    }
}
