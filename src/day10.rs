use std::fs;

/// ```
/// use aoc2022::Device;
///
/// let device = Device::load_from("data/10_test.in").unwrap();
/// assert_eq!(device.sum_of_signals(), Some(13140));
/// assert_eq!(device.screen(), "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n###   ###   ###   ###   ###   ###   ### \n####    ####    ####    ####    ####    \n#####     #####     #####     #####     \n######      ######      ######      ####\n#######       #######       #######     ");
/// ```
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
pub struct Device(Vec<i32>);
impl Device {
    pub fn load_from(path: &str) -> Option<Self> {
        let input = fs::read_to_string(path).ok()?;
        Self::from(&input)
    }

    pub fn from(input: &str) -> Option<Self> {
        let size = input.chars().filter(|c| c == &'\n').count() + 1;
        let mut states = Vec::with_capacity(size * 2);
        states.push(1);
        for line in input.split('\n') {
            Instruction::from(line)?.run(&mut states);
        }
        Some(Self(states))
    }

    pub fn sum_of_signals(&self) -> Option<i32> {
        let steps = vec![20, 60, 100, 140, 180, 220].into_iter();
        let mut result = 0;
        for step in steps {
            result += step as i32 * self.0.get(step - 1)?;
        }
        Some(result)
    }

    pub fn screen(&self) -> String {
        let size = self.0.len() * 41 / 40; // add extra size for newline symbols
        let mut output = String::with_capacity(size);
        for (line, chunk) in self.0.chunks(40).enumerate() {
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
