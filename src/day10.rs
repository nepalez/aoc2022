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
}

#[derive(Debug, Default)]
pub struct Device(Vec<Instruction>);
impl Device {
    pub fn load_from(path: &str) -> Option<Self> {
        let input = fs::read_to_string(path).ok()?;
        Self::from(&input)
    }

    pub fn from(input: &str) -> Option<Self> {
        let size = input.chars().filter(|c| c == &'\n').count() + 1;
        let mut device: Vec<Instruction> = Vec::with_capacity(size);
        for line in input.split('\n') {
            device.push(Instruction::from(line)?);
        }
        Some(Self(device))
    }

    pub fn sum_of_signals(&self) -> Option<i32> {
        let steps = vec![20, 60, 100, 140, 180, 220].into_iter();
        let states = self.run();
        let mut result = 0;
        for step in steps {
            result += step as i32 * states.get(step - 1)?;
        }
        Some(result)
    }

    fn run(&self) -> Vec<i32> {
        let mut states = Vec::with_capacity(1 + self.0.len());
        let mut state = 1;
        for i in self.0.iter() {
            states.push(state);
            if let Instruction::Addx(value) = i {
                states.push(state);
                state += value;
            }
        }
        states.push(state);
        states
    }

    pub fn screen(&self) -> String {
        let steps = self.run();
        let size = steps.len() * 41 / 40; // add extra size for newline symbols
        let mut output = String::with_capacity(size);
        for (line, chunk) in steps.chunks(40).enumerate() {
            if chunk.len() < 40 {
                break;
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
