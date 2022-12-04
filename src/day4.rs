use std::fs;

pub struct Section {
    pub min: u32,
    pub max: u32,
}
impl Section {
    pub fn from(input: &str) -> Option<Self> {
        let mut data = input.split('-');
        let min: u32 = data.next()?.parse().ok()?;
        let max: u32 = data.next()?.parse().ok()?;
        Some(Self { min, max })
    }
}

pub struct Pair(Section, Section);
impl Pair {
    pub fn from(input: &str) -> Option<Self> {
        let mut data = input.split(',');
        let left = Section::from(data.next()?)?;
        let right = Section::from(data.next()?)?;
        Some(Self(left, right))
    }

    /// Is one assignment fully contains the other
    /// ```
    /// use aoc2022::Pair;
    ///
    /// let pair = Pair::from("2-4,6-8").unwrap();
    /// assert_eq!(pair.fully_contained(), false);
    ///
    /// let pair = Pair::from("5-7,7-9").unwrap();
    /// assert_eq!(pair.fully_contained(), false);
    ///
    /// let pair = Pair::from("2-8,3-7").unwrap();
    /// assert_eq!(pair.fully_contained(), true);
    ///
    /// let pair = Pair::from("6-6,4-6").unwrap();
    /// assert_eq!(pair.fully_contained(), true);
    /// ```
    pub fn fully_contained(&self) -> bool {
        (self.0.min <= self.1.min) & (self.0.max >= self.1.max)
            || (self.0.min >= self.1.min) & (self.0.max <= self.1.max)
    }

    /// If assignments overlap
    /// ```
    /// use aoc2022::Pair;
    ///
    /// let pair = Pair::from("5-7,7-9").unwrap();
    /// assert_eq!(pair.overlaps(), true);
    ///
    /// let pair = Pair::from("2-8,3-7").unwrap();
    /// assert_eq!(pair.overlaps(), true);
    ///
    /// let pair = Pair::from("6-6,4-6").unwrap();
    /// assert_eq!(pair.overlaps(), true);
    ///
    /// let pair = Pair::from("2-6,4-8").unwrap();
    /// assert_eq!(pair.overlaps(), true);
    ///
    /// let pair = Pair::from("2-3,4-5").unwrap();
    /// assert_eq!(pair.overlaps(), false);
    /// ```
    pub fn overlaps(&self) -> bool {
        (self.0.min <= self.1.max) & (self.1.min <= self.0.max)
    }
}

pub struct Pairs(Vec<Pair>);
impl Pairs {
    pub fn from(input: &str) -> Option<Self> {
        let list = input.split('\n').map(|s| Pair::from(s));
        let mut pairs: Vec<Pair> = Vec::new();
        for pair in list {
            pairs.push(pair?)
        }
        Some(Self(pairs))
    }

    pub fn load_from(path: &str) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;
        Pairs::from(&data)
    }

    /// Count inclusive pairs where one assignment fully contains the other
    /// ```
    /// use aoc2022::Pairs;
    ///
    /// let pairs = Pairs::from("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8").unwrap();
    /// assert_eq!(pairs.count_fully_contained(), 2);
    /// ```
    pub fn count_fully_contained(&self) -> u32 {
        self.0
            .iter()
            .fold(0, |a, pair| a + if pair.fully_contained() { 1 } else { 0 })
    }

    /// Count inclusive pairs where one assignment fully contains the other
    /// ```
    /// use aoc2022::Pairs;
    ///
    /// let pairs = Pairs::from("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8").unwrap();
    /// assert_eq!(pairs.count_overlapped(), 4);
    /// ```
    pub fn count_overlapped(&self) -> u32 {
        self.0
            .iter()
            .fold(0, |a, pair| a + if pair.overlaps() { 1 } else { 0 })
    }
}
