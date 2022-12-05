use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug, Clone)]
pub struct Stack(VecDeque<char>);
impl Stack {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn push(&mut self, item: char) {
        if item != ' ' {
            self.0.push_front(item)
        }
    }

    pub fn pop(&mut self) -> Option<char> {
        self.0.pop_front()
    }

    pub fn top(&self) -> char {
        self.0.get(0).unwrap_or(&' ').clone()
    }
}

/// ```
/// use aoc2022::Content;
///
/// let content = Content::from("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ").unwrap();
/// assert_eq!(content.top().unwrap(), "NDP");
/// ```
#[derive(Debug, Clone)]
pub struct Content {
    stacks: HashMap<char, Stack>,
    places: HashMap<usize, char>,
    labels: Vec<char>,
}
impl Content {
    // Init stack from line of numbers like: " 1   2   3 "
    fn init_from(line: &str) -> Self {
        let mut stacks = HashMap::new();
        let mut places = HashMap::new();
        let mut labels = Vec::new();
        for (p, c) in line.chars().enumerate() {
            if c != ' ' {
                stacks.insert(c, Stack::new());
                places.insert(p, c);
                labels.push(c);
            }
        }
        Self {
            stacks,
            places,
            labels,
        }
    }

    // Fill stack from line like: "[A] [B] [C]" or "[D]     [E]"
    fn add_tier(&mut self, line: &str) -> Option<()> {
        for (index, item) in line.chars().enumerate() {
            if let Some(c) = self.places.get(&index) {
                self.stacks.get_mut(c)?.push(item);
            }
        }
        Some(())
    }

    // Prepare content from input file header like:
    // [D]     [E]
    // [A] [B] [C]
    //  1   2   3
    pub fn from(header: &str) -> Option<Self> {
        let mut lines: Vec<&str> = header.split('\n').collect();
        lines.reverse();
        let mut lines = lines.into_iter();
        let mut content = Self::init_from(lines.next()?);
        for line in lines {
            content.add_tier(line)?;
        }
        Some(content)
    }

    // Push the item to the corresponding stack
    pub fn push(&mut self, stack: char, item: char) -> Option<()> {
        Some(self.stacks.get_mut(&stack)?.push(item))
    }

    // Pop the item from the corresponding stack
    pub fn pop(&mut self, stack: char) -> Option<char> {
        self.stacks.get_mut(&stack)?.pop()
    }

    // Get the top tier of items
    pub fn top(&self) -> Option<String> {
        let mut output = String::with_capacity(self.labels.len());
        for l in self.labels.iter() {
            output.push(self.stacks.get(l)?.top());
        }
        Some(output)
    }
}

#[derive(Debug)]
pub struct Command(i32, char, char);
impl Command {
    // parse command from line like: move 1 from 2 to 1
    pub fn from(line: &str) -> Option<Self> {
        let mut output = Self(0, ' ', ' ');
        for cap in Regex::new(r"move (\d+) from (\d) to (\d)")
            .ok()?
            .captures_iter(line)
        {
            output.0 = cap[1].parse::<i32>().ok()?;
            output.1 = cap[2].chars().next()?;
            output.2 = cap[3].chars().next()?;
        }
        Some(output)
    }

    /// ```
    /// use aoc2022::{Content, Command};
    ///
    /// let mut content = Content::from("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ").unwrap();
    /// let command = Command::from("move 2 from 2 to 1").unwrap();
    /// command.apply_old(&mut content);
    /// assert_eq!(content.top().unwrap(), "CMP");
    /// ```
    pub fn apply_old(&self, content: &mut Content) -> Option<()> {
        for _ in 0..self.0 {
            let item = { content.pop(self.1)? };
            content.push(self.2, item)?
        }
        Some(())
    }

    /// ```
    /// use aoc2022::{Content, Command};
    ///
    /// let mut content = Content::from("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ").unwrap();
    /// let command = Command::from("move 2 from 2 to 1").unwrap();
    /// command.apply_new(&mut content);
    /// assert_eq!(content.top().unwrap(), "DMP");
    /// ```
    pub fn apply_new(&self, content: &mut Content) -> Option<()> {
        let mut buffer: Vec<char> = Vec::new();
        for _ in 0..self.0 {
            buffer.push(content.pop(self.1)?);
        }
        for _ in 0..self.0 {
            content.push(self.2, buffer.pop()?)?;
        }
        Some(())
    }
}

/// ```
/// use aoc2022::Crane;
///
/// let data = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\n\
/// move 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
///
/// let crane = Crane::from(data).unwrap();
/// assert_eq!(crane.apply_old().unwrap(), "CMZ");
/// assert_eq!(crane.apply_new().unwrap(), "MCD");
/// ```
pub struct Crane {
    content: Content,
    commands: Vec<Command>,
}
impl Crane {
    pub fn load_from(path: &str) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;
        Self::from(&data)
    }

    pub fn from(data: &str) -> Option<Self> {
        let mut parts = data.split("\n\n");
        let content = Content::from(parts.next()?)?;

        let mut commands: Vec<Command> = Vec::new();
        for p in parts.next()?.split('\n') {
            commands.push(Command::from(p)?)
        }

        Some(Self { content, commands })
    }

    pub fn apply_old(&self) -> Option<String> {
        let mut content = self.content.clone();
        for c in self.commands.iter() {
            c.apply_old(&mut content)?;
        }
        content.top()
    }

    pub fn apply_new(&self) -> Option<String> {
        let mut content = self.content.clone();
        for c in self.commands.iter() {
            c.apply_new(&mut content)?;
        }
        content.top()
    }
}
