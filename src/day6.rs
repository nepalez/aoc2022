use std::borrow::BorrowMut;
use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug)]
pub struct Buffer(VecDeque<char>);
impl Buffer {
    pub fn from(input: &str, size: usize) -> Option<Self> {
        let mut vd = VecDeque::new();
        let mut input = input.chars();
        for _ in 0..size {
            vd.push_back(input.next()?)
        }
        Some(Self(vd))
    }

    pub fn forward(&mut self, item: char) {
        self.0.pop_front();
        self.0.push_back(item);
    }

    pub fn is_uniq(&self) -> bool {
        let mut hash: HashSet<&char> = HashSet::with_capacity(self.0.len());
        for c in self.0.iter() {
            hash.insert(c);
        }
        hash.len() == self.0.len()
    }
}

pub struct Stream(pub String);
impl Stream {
    pub fn load_from(path: &str) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;
        Some(Self(data))
    }

    /// ```
    /// use aoc2022::Stream;
    ///
    /// let stream = Stream("mjqjpqmgbljsphdztnvjfqwrcgsmlb".into());
    /// assert_eq!(stream.start_packet(), Some(7));
    /// assert_eq!(stream.start_message(), Some(19));
    ///
    /// let stream = Stream("bvwbjplbgvbhsrlpgdmjqwftvncz".into());
    /// assert_eq!(stream.start_packet(), Some(5));
    /// assert_eq!(stream.start_message(), Some(23));
    ///
    /// let stream = Stream("nppdvjthqldpwncqszvftbrmjlhg".into());
    /// assert_eq!(stream.start_packet(), Some(6));
    /// assert_eq!(stream.start_message(), Some(23));
    ///
    /// let stream = Stream("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".into());
    /// assert_eq!(stream.start_packet(), Some(10));
    /// assert_eq!(stream.start_message(), Some(29));
    ///
    /// let stream = Stream("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".into());
    /// assert_eq!(stream.start_packet(), Some(11));
    /// assert_eq!(stream.start_message(), Some(26));
    /// ```
    pub fn start_packet(&self) -> Option<usize> {
        self.find_uniq_seq(4)
    }

    pub fn start_message(&self) -> Option<usize> {
        self.find_uniq_seq(14)
    }

    fn find_uniq_seq(&self, size: usize) -> Option<usize> {
        let mut buffer = Buffer::from(&self.0, size)?;
        for (index, c) in self.0.chars().enumerate() {
            if buffer.is_uniq() {
                return Some(index);
            }
            buffer.borrow_mut().forward(c);
        }
        None
    }
}
