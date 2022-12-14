use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
pub enum Material {
    Rock,
    Sand,
}

#[derive(Debug, Clone)]
pub enum SandState {
    Fall,
    Rest,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
impl Position {
    pub fn from_str(input: &str) -> Option<Self> {
        let mut list = input.split(',').into_iter();
        let x = list.next()?.parse().ok()?;
        let y = list.next()?.parse().ok()?;
        Some(Self { x, y })
    }
    
    pub fn down(&mut self) -> bool {
        self.y += 1;
        true
    }

    pub fn left(&mut self) -> bool {
        self.x -= 1;
        true
    }

    pub fn right(&mut self) -> bool {
        self.x += 2;
        true
    }
}

#[derive(Debug)]
pub struct SandUnit {
    pub position: Position,
    pub state: SandState,
}
impl SandUnit {
    // Drop the nex sand unit
    pub fn new() -> Self {
        Self { position: Position { x: 500, y: 0 }, state: SandState::Fall }
    }
    
    pub fn fall_into(cave: &Cave) -> Self {
        let mut unit = Self::new();
        loop {
            match unit.state {
                SandState::Fall => unit.fly_in(cave),
                _ => break,
            }
        }
        unit
    }

    pub fn fly_in(&mut self, cave: &Cave) {
        if let SandState::Fall = self.state {
            let mut pos = self.position.clone();
            if pos.y > cave.bottom {
                self.state = SandState::Rest;
            } else if pos.down() && cave.fill.get(&pos).is_none() {
                self.position = pos;
            } else if pos.left() && cave.fill.get(&pos).is_none() {
                self.position = pos;
            } else if pos.right() && cave.fill.get(&pos).is_none() {
                self.position = pos;
            } else {
                self.state = SandState::Rest;
            }
        }
    }
}

#[derive(Debug)]
pub struct Cave {
    fill: HashMap<Position, Material>,
    left: usize,
    right: usize,
    bottom: usize,
}

impl Cave {
    pub fn load_from(path: &str) -> Option<Self> {
        let input = fs::read_to_string(path).ok()?;
        Some(Self::from_str(&input)?)
    }
    
    pub fn from_str(input: &str) -> Option<Self> {
        let mut cave = Self { fill: HashMap::new(), left: 500, right: 500, bottom: 0 };

        for line in input.split('\n') {
            let mut list = line.split(" -> ");
            if let Some(Some(mut prev)) = list.next().map(|p| Position::from_str(p)) {
                while let Some(Some(next)) = list.next().map(|p| Position::from_str(p)) {
                    for x in prev.x.min(next.x)..=prev.x.max(next.x) {
                        for y in prev.y.min(next.y)..=prev.y.max(next.y) {
                            let pos = Position { x, y };
                            cave.fill.insert(pos, Material::Rock);
                            cave.bottom = cave.bottom.max(y);
                            cave.left = cave.left.min(x);
                            cave.right = cave.right.max(x);
                        }
                    }
                    prev = next;
                }
            }
        }

        Some(cave)
    }
    
    pub fn pour_sand(&mut self) {
        loop {
            let unit = SandUnit::fall_into(&self);
            self.fill.insert(unit.position.clone(), Material::Sand);
            if unit.position.x == 500 && unit.position.y == 0 {
                break;
            }
        }
    }
    
    pub fn count_sand_units(&self) -> usize {
        self.fill.iter().filter(|(_, unit)| if let Material::Sand = unit { true } else { false }).count()
    }

    pub fn print(&self) {
        for y in 0..=self.bottom + 3 {
            let mut line = String::new();
            for x in (self.left - 1)..=(self.right + 1) {
                let item = match self.fill.get(&Position { x, y }) {
                    Some(Material::Rock) => '#',
                    Some(Material::Sand) => 'o',
                    None => '.',
                };
                if y == 0 && x == 500 {
                    line.push('+');
                } else {
                    line.push(item);
                }
            }
            println!("{}", line);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test() {
        let mut cave = Cave::load_from("data/14_test.in").unwrap();
        cave.pour_sand();
        assert_eq!(cave.count_sand_units(), 93);
    }
}