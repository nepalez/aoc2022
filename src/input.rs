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
    
    /// Load rock-paper-scissors input strategy from a file
    pub fn load_strategy(path: &str) -> Option<Vec<(char, char)>> {
        let data = fs::read_to_string(path).ok()?;
        
        let output: Vec<(char, char)> = data.split('\n').map(|i| {
            let mut chars = i.chars();
            let a = chars.next().unwrap();
            chars.next();
            let b = chars.next().unwrap();
            (a, b)
        }).collect();
        
        Some(output)
    }
}
