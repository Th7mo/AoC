use crate::tree::Tree;

pub struct Forest {
    matrix: Vec<Vec<Tree>>,
}

impl Forest {
    pub fn new() -> Self {
        Forest { matrix: Vec::new() }
    }

    pub fn add_row(&mut self, row: Vec<Tree>) {
        self.matrix.push(row);
    }

    pub fn visible_trees(&mut self) -> u32 {
        self.iterate_to_left();
        self.iterate_to_right();
        self.iterate_to_down();
        self.iterate_to_up();
        self.amount_of_visible_trees()
    }

    fn iterate_to_right(&mut self) {
        for row in self.matrix.iter_mut() {
            Self::update_trees(row);
        }
    }

    fn iterate_to_left(&mut self) {
        for row in self.matrix.iter_mut().rev() {
            Self::update_trees(row);
        }
    }

    fn iterate_to_down(&mut self) {
        let mut highest_tree_height = 0;
        let size = self.matrix.len();
        for i in 0..size {
            for j in 0..size {
                let tree = self.matrix.get_mut(j).unwrap().get_mut(i).unwrap();
                highest_tree_height = Self::update_tree(tree, highest_tree_height);
            }
        }
    }

    fn iterate_to_up(&mut self) {
        let mut highest_tree_height = 0;
        let size = self.matrix.len();
        for i in 0..size {
            for j in size..0 {
                let tree = self.matrix.get_mut(j).unwrap().get_mut(i).unwrap();
                highest_tree_height = Self::update_tree(tree, highest_tree_height);
            }
        }
    }

    fn update_trees(row: &mut Vec<Tree>) {
        let mut highest_tree_height = 0;
        let size = row.len();
        for (index, tree) in row.iter_mut().enumerate() {
            if index == 0 || index == size - 1 {
                continue;
            }
            Self::update_tree(tree, highest_tree_height);
        }
    }

    fn update_tree(tree: &mut Tree, highest: u8) -> u8 {
        if tree.visible {
            return highest;
        }

        if tree.higher_than(highest) {
            tree.visible = true;
            return tree.height;
        }

        highest
    }

    fn amount_of_visible_trees(&self) -> u32 {
        let mut visible_trees = 0;
        for row in &self.matrix {
            for tree in row {
                if tree.visible {
                    visible_trees += 1;
                }
            }
        }
        visible_trees
    }
}