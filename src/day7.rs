use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub size: i32,
    pub parent: Option<Tree>,
    pub branches: Vec<Tree>,
}

#[derive(Debug)]
pub struct Tree(Rc<RefCell<Node>>);
impl Tree {
    pub fn space_to_drop(&self, space_to_get: i32) -> Option<i32> {
        let extra_space = self.size() - space_to_get;
        self.folder_sizes()
            .iter()
            .find(|&&a| a >= extra_space)
            .map(|&i| i)
    }

    pub fn sum_of_folders_up_to(&self, limit: i32) -> i32 {
        self.folder_sizes()
            .iter()
            .filter(|&&i| i <= limit)
            .fold(0, |a, &i| a + i)
    }

    // build a file tree from given input file
    pub fn load_from(path: &str) -> Option<Self> {
        let data = fs::read_to_string(path).ok()?;

        let mut lines = data.split('\n');
        if lines.next()? != "$ cd /" {
            return None;
        }

        let mut root = Self::new("/", 0);
        let mut current = root.copy();

        for line in lines {
            if line == "$ cd .." {
                current = current.parent()?
            } else if line.len() > 5 && &line[0..5] == "$ cd " {
                current = Self::insert(&mut current, &line[5..], 0);
            } else if line.len() > 3 && &line[0..1] != "$" && &line[0..3] != "dir" {
                let mut parts = line.split(' ');
                let size = parts.next()?.parse().ok()?;
                let name = parts.next()?;
                Self::insert(&mut current, name, size);
            }
        }
        root.finalize();

        Some(root)
    }

    // the size of the root node
    pub fn size(&self) -> i32 {
        Rc::clone(&self.0).borrow().size
    }

    // the plain list of folder sizes
    pub fn folder_sizes(&self) -> Vec<i32> {
        let mut output = vec![];

        let node = self.0.borrow();
        if !node.branches.is_empty() {
            output.push(node.size);
            for branch in Rc::clone(&self.0).borrow().branches.iter() {
                for item in branch.folder_sizes() {
                    output.push(item);
                }
            }
        }

        if let None = node.parent {
            output.sort_by(|a, b| a.cmp(&b));
        }
        output
    }

    fn new(name: &str, size: i32) -> Self {
        let name = name.into();
        Self(Rc::new(RefCell::new(Node {
            name,
            size,
            parent: None,
            branches: vec![],
        })))
    }

    fn copy(&self) -> Self {
        Self(Rc::clone(&self.0))
    }

    fn insert(&self, name: &str, size: i32) -> Self {
        let child = Self::new(name, size);
        self.0.borrow_mut().branches.push(child.copy());
        {
            let mut mut_child = child.0.borrow_mut();
            mut_child.parent = Some(self.copy());
        }
        child
    }

    fn parent(&self) -> Option<Self> {
        let current_clone = Rc::clone(&self.0);
        let borrow = current_clone.borrow();
        Some(borrow.parent.as_ref()?.copy())
    }

    fn finalize(&mut self) -> i32 {
        let clone = Rc::clone(&self.0);
        let mut node = clone.borrow_mut();
        if !node.branches.is_empty() {
            node.size = {
                node.branches
                    .iter_mut()
                    .map(|b| b.finalize())
                    .fold(0, |a, i| a + i)
            };
        }
        node.size
    }
}
