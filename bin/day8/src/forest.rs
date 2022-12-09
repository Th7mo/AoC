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
        self.iterate_to_right();
        self.iterate_to_left();
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
        let size = self.matrix.len();
        for i in 0..size {
            let mut highest_tree_height = 0;
            for j in (0..size).rev() {
                highest_tree_height = self.update_tree_column(i, j, highest_tree_height);
            }
        }
    }

    fn iterate_to_down(&mut self) {
        let size = self.matrix.len();
        for row in 0..size {
            let mut highest_tree_height = 0;
            for column in 0..size {
                highest_tree_height = self.update_tree_column(column, row, highest_tree_height);
            }
        }
    }

    fn iterate_to_up(&mut self) {
        let size = self.matrix.len();
        for i in 0..size {
            let mut highest_tree_height = 0;
            for j in (0..size).rev() {
                highest_tree_height = self.update_tree_column(j, i, highest_tree_height);
            }
        }
    }

    fn update_tree_column(&mut self, j: usize, i: usize, highest: u8) -> u8 {
        let size = self.matrix.len();
        let tree = self.matrix.get_mut(j).unwrap().get_mut(i).unwrap();
        if j == 0 || j == size - 1 || i == 0 || i == size - 1 {
            tree.visible = true;
        }
        Self::update_tree(tree, highest)
    }

    fn update_trees(row: &mut Vec<Tree>) {
        let mut highest_tree_height = 0;
        let size = row.len();
        for (index, tree) in row.iter_mut().enumerate() {
            if index == 0 || index == size - 1 {
                tree.visible = true;
            }
            highest_tree_height = Self::update_tree(tree, highest_tree_height);
        }
    }

    fn update_tree(tree: &mut Tree, highest: u8) -> u8 {
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

    pub fn highest_scenic_score(&mut self) -> u32 {
        self.set_scenic_scores();
        let mut highest_scenic_score = 0;
        for row in &self.matrix {
            for tree in row {
                if tree.scenic_score > highest_scenic_score {
                    highest_scenic_score = tree.scenic_score;
                }
            }
        }
        highest_scenic_score
    }

    fn set_scenic_scores(&mut self) {
        let size = self.matrix.len();
        for row in 0..size {
            for col in 0..size {
                self.calc_scenic_score_for_tree(row, col);
            }
        }
    }

    fn calc_scenic_score_for_tree(&mut self, row: usize, col: usize) {
        let size = self.matrix.len();
        let tree = self.matrix.get(row).unwrap().get(col).unwrap();

        let mut scenic_score_to_right = 0;
        for col_copy in col..size {
            if col_copy == col {
                continue;
            }
            scenic_score_to_right += 1;
            let next_tree = self.matrix.get(row).unwrap().get(col_copy).unwrap();
            if !tree.higher_than(next_tree.height) {
                break;
            }
        }

        let mut scenic_score_to_left = 0;
        for col_copy in (0..col).rev() {
            if col_copy == col {
                continue;
            }
            scenic_score_to_left += 1;
            let next_tree = self.matrix.get(row).unwrap().get(col_copy).unwrap();
            if !tree.higher_than(next_tree.height) {
                break;
            }
        }

        let mut scenic_score_to_down = 0;
        for row_copy in row..size {
            if row_copy == row {
                continue;
            }
            scenic_score_to_down += 1;
            let next_tree = self.matrix.get(row_copy).unwrap().get(col).unwrap();
            if !tree.higher_than(next_tree.height) {
                break;
            }
        }

        let mut scenic_score_to_up = 0;
        for row_copy in (0..row).rev() {
            if row_copy == row {
                continue;
            }
            scenic_score_to_up += 1;
            let next_tree = self.matrix.get(row_copy).unwrap().get(col).unwrap();
            if !tree.higher_than(next_tree.height) {
                break;
            }
        }
        let tree = self.matrix.get_mut(row).unwrap().get_mut(col).unwrap();
        tree.scenic_score = scenic_score_to_right
            * scenic_score_to_left
            * scenic_score_to_up
            * scenic_score_to_down;
    }
}
