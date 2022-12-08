use std::fs;

#[derive(Debug, Default)]
struct Tree {
    pub size: i32,
    pub visible: bool,
    pub score: i32,
}
impl Tree {
    pub fn new(size: i32) -> Self {
        Self {
            size,
            visible: false,
            score: 1,
        }
    }

    pub fn from(input: char) -> Option<Self> {
        let size = input.to_digit(10)? as i32;
        Some(Self::new(size))
    }

    pub fn record_observation(&mut self, visible_trees: i32, view_blocked: bool) {
        self.visible |= !view_blocked;
        self.score *= visible_trees;
    }
}

/// The forest as a forest of trees
/// ```
/// use aoc2022::Forest;
///
/// let forest = Forest::from("30373\n25512\n65332\n33549\n35390").unwrap();
/// assert_eq!(forest.count_visible(), 21);
/// assert_eq!(forest.best_score(), 8);
/// ```
#[derive(Debug, Default)]
pub struct Forest {
    height: usize,
    width: usize,
    trees: Vec<Vec<Tree>>,
}
impl Forest {
    /// The number of visible trees
    pub fn count_visible(&self) -> i32 {
        let mut result = 0;
        for row in self.trees.iter() {
            for tree in row.iter() {
                if tree.visible {
                    result += 1;
                }
            }
        }
        result
    }

    /// The highest score of the trees
    pub fn best_score(&self) -> i32 {
        let mut score = 0;
        for row in self.trees.iter() {
            for tree in row.iter() {
                if tree.score > score {
                    score = tree.score
                }
            }
        }
        score
    }

    /// Load the forest from file by path
    pub fn load_from(path: &str) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;
        Self::from(&data)
    }

    /// Build the forest from str
    pub fn from(input: &str) -> Option<Self> {
        let (height, width) = Self::sizes(input)?;
        let mut forest = Forest::init(height, width);
        forest.load_data(input)?;
        forest.observe_trees()?;
        Some(forest)
    }

    // return sizes of the input if it is correct
    fn sizes(input: &str) -> Option<(usize, usize)> {
        let mut width = 0;
        let mut height = 0;
        for (index, line) in input.split('\n').enumerate() {
            if index == 0 {
                width = line.len();
            } else if width != line.len() {
                return None; // expect all rows have the same size
            }
            height += 1;
        }
        Some((height, width))
    }

    // initialize the empty forest
    fn init(height: usize, width: usize) -> Self {
        let mut trees = Vec::with_capacity(height);
        for _ in 0..height {
            trees.push(Vec::with_capacity(width));
        }
        Self {
            height,
            width,
            trees,
        }
    }

    // fill forest from data
    fn load_data(&mut self, data: &str) -> Option<()> {
        for (row, line) in data.split('\n').enumerate() {
            for (_, size) in line.chars().enumerate() {
                let tree = Tree::from(size)?;
                self.trees.get_mut(row)?.push(tree);
            }
        }
        Some(())
    }

    // finalize trees visibility and scores
    fn observe_trees(&mut self) -> Option<()> {
        for row in 0..self.height {
            for col in 0..self.width {
                // row, col -- inner only
                let size = self.tree(row, col)?.size.clone();

                // look up
                let mut visible_trees = 0;
                let mut view_blocked = false;
                for i in 1..=row {
                    visible_trees += 1;
                    if self.tree(row - i, col)?.size >= size {
                        view_blocked = true;
                        break;
                    }
                }
                self.mut_tree(row, col)?
                    .record_observation(visible_trees, view_blocked);

                // look to left
                let mut visible_trees = 0;
                let mut view_blocked = false;
                for j in 1..=col {
                    visible_trees += 1;
                    if self.tree(row, col - j)?.size >= size {
                        view_blocked = true;
                        break;
                    }
                }
                self.mut_tree(row, col)?
                    .record_observation(visible_trees, view_blocked);

                // look down
                let mut visible_trees = 0;
                let mut view_blocked = false;
                for i in (row + 1)..self.width {
                    visible_trees += 1;
                    if self.tree(i, col)?.size >= size {
                        view_blocked = true;
                        break;
                    }
                }
                self.mut_tree(row, col)?
                    .record_observation(visible_trees, view_blocked);

                // look down
                let mut visible_trees = 0;
                let mut view_blocked = false;
                for j in (col + 1)..self.width {
                    visible_trees += 1;
                    if self.tree(row, j)?.size >= size {
                        view_blocked = true;
                        break;
                    }
                }
                self.mut_tree(row, col)?
                    .record_observation(visible_trees, view_blocked);
            }
        }
        Some(())
    }

    // methods to access trees

    fn tree(&self, row: usize, col: usize) -> Option<&Tree> {
        self.trees.get(row)?.get(col)
    }

    fn mut_tree(&mut self, row: usize, col: usize) -> Option<&mut Tree> {
        self.trees.get_mut(row)?.get_mut(col)
    }
}
