use std::fs;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Noop,
    Addx(i32),
}
impl Instruction {
    pub fn from(input: &str) -> Option<Self> {
        if input == "noop" {
            Some(Instruction::Noop)
        } else if input.len() > 5 && &input[0..4] == "addx" {
            Some(Instruction::Addx(input[5..].parse::<i32>().ok()?))
        } else {
            None
        }
    }

    pub fn run(&self, states: &mut Vec<i32>) {
        let state = states.last().unwrap_or(&1).clone();
        if let Self::Addx(value) = self {
            states.push(state.clone());
            states.push(state + value);
        } else {
            states.push(state);
        }
    }
}

#[derive(Debug, Default)]
pub struct Device {
    states: Vec<i32>,
    size: usize,
}
impl Device {
    pub fn load_from(path: &str) -> Option<Self> {
        Self::from(&fs::read_to_string(path).ok()?)
    }

    pub fn from(input: &str) -> Option<Self> {
        let mut size = input.chars().filter(|c| c == &'\n').count() * 2 + 3;
        let mut states = Vec::with_capacity(size);

        states.push(1);
        for line in input.split('\n') {
            Instruction::from(line)?.run(&mut states);
        }
        size = states.len();

        Some(Self { states, size })
    }

    pub fn sum_of_signals(&self) -> Option<i32> {
        let steps = vec![20, 60, 100, 140, 180, 220].into_iter();
        let mut result = 0;
        for step in steps {
            result += step as i32 * self.states.get(step - 1)?;
        }
        Some(result)
    }

    pub fn screen(&self) -> String {
        let mut output = String::with_capacity(self.size * 41 / 40);
        for (line, chunk) in self.states.chunks(40).enumerate() {
            if chunk.len() < 40 {
                break; // only full lines are visible
            }
            if line > 0 {
                output.push('\n');
            }
            for (pos, &x) in chunk.iter().enumerate() {
                let pos = pos as i32;
                output.push(if pos > x - 2 && pos < x + 2 { '#' } else { ' ' });
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_signals_works() {
        let device = Device::load_from("data/10_test.in").unwrap();
        assert_eq!(device.sum_of_signals(), Some(13140));
    }

    #[test]
    fn screen_works() {
        let device = Device::load_from("data/10_test.in").unwrap();
        let expected = fs::read_to_string("data/10_test.out").unwrap();
        let actual = device.screen();
        println!("EXPECTED:\n{}\n\nACTUAL:\n{}", expected, actual);
        assert_eq!(expected, actual);
    }
}
