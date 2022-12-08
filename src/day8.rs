use std::fs;

#[derive(Debug, Default)]
pub struct Tree {
    pub size: i32,
    pub visible: bool,
    pub score: i32,
}

#[derive(Debug, Default)]
pub struct Grid {
    height: usize,
    width: usize,
    pub trees: Vec<Vec<Tree>>,
}

/// ```
/// use aoc2022::Grid;
///
/// let grid = Grid::from("30373\n25512\n65332\n33549\n35390").unwrap();
/// assert_eq!(grid.count_visible(), 21);
/// assert_eq!(grid.best_score(), 8);
/// ```
impl Grid {
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

    pub fn load_from(path: &str) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;
        Self::from(&data)
    }

    pub fn from(input: &str) -> Option<Self> {
        // prepare grid
        let mut width = 0;
        let mut height = 0;
        for line in input.split('\n') {
            width = line.len();
            height += 1;
        }
        let mut grid = Grid::init(height, width);

        // fill it with data
        for (row, line) in input.split('\n').enumerate() {
            for (col, char) in line.chars().enumerate() {
                grid.insert(row, col, char.to_digit(10)? as i32);
            }
        }

        grid.top_view();
        grid.left_view();
        grid.bottom_view();
        grid.right_view();
        grid.count_scores();

        Some(grid)
    }

    fn print_visibility(&self) {
        for row in self.trees.iter() {
            let mut out = String::from("");
            for tree in row.iter() {
                if tree.visible {
                    out.push('1');
                } else {
                    out.push('0');
                }
            }
            println!("{}", out);
        }
        println!("-----");
    }

    fn init(height: usize, width: usize) -> Self {
        let mut trees = Vec::with_capacity(height);
        for i in 0..height {
            trees.push(Vec::with_capacity(width));
        }
        Self {
            height,
            width,
            trees,
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<&Tree> {
        self.trees.get(row)?.get(col)
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut Tree> {
        self.trees.get_mut(row)?.get_mut(col)
    }

    fn insert(&mut self, row: usize, col: usize, size: i32) -> Option<()> {
        let tree = Tree {
            size,
            visible: false,
            score: 0,
        };
        self.trees.get_mut(row)?.insert(col, tree);
        Some(())
    }

    fn top_view(&mut self) -> Option<()> {
        let mut line: Vec<i32> = Vec::with_capacity(self.width);
        for _ in 0..self.width {
            line.push(-1);
        }

        for i in 0..self.width {
            for j in 0..self.height {
                let tree = self.get_mut(i, j)?;
                if line.get(j)? >= &tree.size {
                    continue;
                }
                tree.visible |= true;
                line[j] = tree.size;
            }
        }
        Some(())
    }

    fn left_view(&mut self) -> Option<()> {
        let mut line: Vec<i32> = Vec::with_capacity(self.height);
        for _ in 0..self.height {
            line.push(-1);
        }

        for j in 0..self.height {
            for i in 0..self.width {
                let tree = self.get_mut(i, j)?;
                if line.get(i)? >= &tree.size {
                    continue;
                }
                tree.visible |= true;
                line[i] = tree.size;
            }
        }
        Some(())
    }

    fn right_view(&mut self) -> Option<()> {
        let mut line: Vec<i32> = Vec::with_capacity(self.height);
        for _ in 0..self.height {
            line.push(-1);
        }

        for j in 0..self.height {
            let j = self.height - j - 1;
            for i in 0..self.width {
                let tree = self.get_mut(i, j)?;
                if line.get(i)? >= &tree.size {
                    continue;
                }
                tree.visible |= true;
                line[i] = tree.size;
            }
        }
        Some(())
    }

    fn bottom_view(&mut self) -> Option<()> {
        let mut line: Vec<i32> = Vec::with_capacity(self.width);
        for _ in 0..self.width {
            line.push(-1);
        }

        for i in 0..self.width {
            let i = self.height - i - 1;
            for j in 0..self.height {
                let tree = self.get_mut(i, j)?;
                if line.get(j)? >= &tree.size {
                    continue;
                }
                tree.visible |= true;
                line[j] = tree.size;
            }
        }
        Some(())
    }

    fn count_scores(&mut self) -> Option<()> {
        for row in 1..(self.height - 1) {
            for col in 1..(self.width - 1) {
                // row, col -- inner only
                let size = self.get(row, col)?.size.clone();
                let mut score = 1;

                // look up
                let mut mult = 0;
                for i in 1..=row {
                    mult += 1;
                    if self.get(row - i, col)?.size >= size {
                        break;
                    }
                }
                score *= mult;

                // look to left
                let mut mult = 0;
                for j in 1..=col {
                    mult += 1;
                    if self.get(row, col - j)?.size >= size {
                        break;
                    }
                }
                score *= mult;

                // look down
                let mut mult = 0;
                for i in (row + 1)..self.width {
                    mult += 1;
                    if self.get(i, col)?.size >= size {
                        break;
                    }
                }
                score *= mult;

                // look down
                let mut mult = 0;
                for j in (col + 1)..self.width {
                    mult += 1;
                    if self.get(row, j)?.size >= size {
                        break;
                    }
                }
                score *= mult;

                self.get_mut(row, col)?.score = score;
            }
        }
        Some(())
    }
}
