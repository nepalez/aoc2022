use std::fs;

/// A number of calories tha provide an energy to an elf.
/// Not to be confused with other values like the number of songs an elf knows,
/// because a song can lift his the spirit but cannot feed his body.
pub type Calories = u32;

/// A food supply containing calories to help elves not starve away 
pub struct Supply {
    pub calories: Calories,
}

/// An elf carrying some supplies on his/her narrow back
/// ```
/// use aoc2022::Elf;
/// 
/// let calories: Vec<u32> = [1000, 2000, 3000].into();
/// let elf = Elf::new(calories);
/// assert_eq!(elf.supplied_calories(), 6000 as u32);
/// ```
pub struct Elf {
    supplies: Vec<Supply>,
}
impl Elf {
    pub fn new(c: Vec<u32>) -> Self {
        let supplies: Vec<Supply> = c.into_iter().map(|calories| Supply { calories }).collect();
        Self { supplies }
    }

    /// The number of calories supplied by an elf
    pub fn supplied_calories(&self) -> Calories {
        self.supplies.iter().map(|s| s.calories).reduce(|a, s| a + s).unwrap()
    }
}

/// The band of elves looking for start fruits
pub struct Elves(Vec<Elf>);
impl Elves {
    pub fn load_from(path: &str) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;

        let cargo: Vec<Vec<u32>> = data.split("\n\n").map(|i| {
            i.split('\n').map(|i| i.parse().unwrap()).collect()
        }).collect();

        Some(Self::new(cargo))
    }

    pub fn new(c: Vec<Vec<u32>>) -> Self {
        let elves: Vec<Elf> = c.into_iter().map(|c| Elf::new(c)).collect();
        Self(elves)
    }
    
    fn sort_by_supply(&mut self) {
        self.0.sort_by(|a, b| b.supplied_calories().cmp(&a.supplied_calories()));
    }

    /// Return total calories carried by the n strongest elves
    /// ```
    /// use aoc2022::Elves;
    ///
    /// let cargo = Vec::from([
    ///   Vec::from([1000, 2000, 3000]),
    ///   Vec::from([4000]),
    ///   Vec::from([5000, 6000]),
    ///   Vec::from([7000, 8000, 9000]),
    ///   Vec::from([10000]),
    /// ]);
    /// let mut elves = Elves::new(cargo);
    /// assert_eq!(elves.total_calories_by_top(3), 45000 as u32);
    /// ```
    pub fn total_calories_by_top(&mut self, number: usize) -> Calories {
        self.sort_by_supply();
        self.0
            .iter()
            .take(number)
            .map(|e| e.supplied_calories())
            .reduce(|a, c| a + c)
            .unwrap()
    }

    /// Return calories supplied by the strongest elf
    /// ```
    /// use aoc2022::Elves;
    ///
    /// let cargo = Vec::from([
    ///   Vec::from([1000, 2000, 3000]),
    ///   Vec::from([4000]),
    ///   Vec::from([5000, 6000]),
    ///   Vec::from([7000, 8000, 9000]),
    ///   Vec::from([10000]),
    /// ]);
    /// let mut elves = Elves::new(cargo);
    /// assert_eq!(elves.maximum_supply(), 24000 as u32);
    /// ```
    pub fn maximum_supply(&mut self) -> Calories {
        self.total_calories_by_top(1)
    }
}
