use std::fs;

pub struct Input {}
impl Input {
    /// Load elves' cargo from a file 
    pub fn load_cargo(path: &str) -> Option<Vec<Vec<u32>>> {
        let data = fs::read_to_string(path).ok()?;

        let cargo: Vec<Vec<u32>> = data.split("\n\n").map(|i| {
          i.split('\n').map(|i| i.parse().unwrap()).collect()
        }).collect();

        Some(cargo)
    }
}
