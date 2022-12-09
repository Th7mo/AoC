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
        let size = self.size();
        for row in self.matrix.iter_mut() {
            Self::update_trees(row, size);
        }
    }

    fn iterate_to_left(&mut self) {
        for i in 0..self.size() {
            let mut highest_tree_height = 0;
            for j in (0..self.size()).rev() {
                highest_tree_height = self.update_tree_column(i, j, highest_tree_height);
            }
        }
    }

    fn iterate_to_down(&mut self) {
        for row in 0..self.size() {
            let mut highest_tree_height = 0;
            for column in 0..self.size() {
                highest_tree_height = self.update_tree_column(column, row, highest_tree_height);
            }
        }
    }

    fn iterate_to_up(&mut self) {
        for i in 0..self.size() {
            let mut highest_tree_height = 0;
            for j in (0..self.size()).rev() {
                highest_tree_height = self.update_tree_column(j, i, highest_tree_height);
            }
        }
    }

    fn update_tree_column(&mut self, row: usize, col: usize, highest: u8) -> u8 {
        if self.is_edge_tree(row, col) {
            let tree = self.get_mut_tree_at(row, col);
            tree.visible = true;
        }

        let tree = self.get_mut_tree_at(row, col);
        Self::update_tree(tree, highest)
    }

    fn is_edge_tree(&self, row: usize, col: usize) -> bool {
        row == 0 || row == self.size() - 1 || col == 0 || col == self.size() - 1
    }

    fn update_trees(row: &mut [Tree], size: usize) {
        let mut highest_tree_height = 0;
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
        for row in 0..self.size() {
            for col in 0..self.size() {
                self.calc_scenic_score_for_tree(row, col);
            }
        }
    }

    fn calc_scenic_score_for_tree(&mut self, row: usize, col: usize) {
        let scenic_score = self.scenic_score_to_right(row, col)
            * self.scenic_score_to_left(row, col)
            * self.scenic_score_to_down(row, col)
            * self.scenic_score_to_up(row, col);
        let tree = self.get_mut_tree_at(row, col);
        tree.scenic_score = scenic_score;
    }

    fn scenic_score_to_right(&self, row: usize, col: usize) -> u32 {
        let tree = self.get_tree_at(row, col);
        let mut scenic_score_to_right = 0;
        for col_copy in col..self.size() {
            if col_copy == col {
                continue;
            }
            scenic_score_to_right += 1;
            let next_tree = self.get_tree_at(row, col_copy);
            if tree.smaller_or_equal_to(next_tree) {
                break;
            }
        }
        scenic_score_to_right
    }

    fn scenic_score_to_left(&self, row: usize, col: usize) -> u32 {
        let tree = self.get_tree_at(row, col);
        let mut scenic_score_to_left = 0;
        for col_copy in (0..col).rev() {
            if col_copy == col {
                continue;
            }
            scenic_score_to_left += 1;
            let next_tree = self.get_tree_at(row, col_copy);
            if tree.smaller_or_equal_to(next_tree) {
                break;
            }
        }
        scenic_score_to_left
    }

    fn scenic_score_to_down(&self, row: usize, col: usize) -> u32 {
        let tree = self.get_tree_at(row, col);
        let mut scenic_score_to_down = 0;
        for row_copy in row..self.size() {
            if row_copy == row {
                continue;
            }
            scenic_score_to_down += 1;
            let next_tree = self.get_tree_at(row_copy, col);
            if tree.smaller_or_equal_to(next_tree) {
                break;
            }
        }
        scenic_score_to_down
    }

    fn scenic_score_to_up(&self, row: usize, col: usize) -> u32 {
        let tree = self.get_tree_at(row, col);
        let mut scenic_score_to_up = 0;
        for row_copy in (0..row).rev() {
            if row_copy == row {
                continue;
            }
            scenic_score_to_up += 1;
            let next_tree = self.get_tree_at(row_copy, col);
            if tree.smaller_or_equal_to(next_tree) {
                break;
            }
        }
        scenic_score_to_up
    }

    fn get_tree_at(&self, row: usize, col: usize) -> &Tree {
        let Some(row) = self.matrix.get(row) else {
            panic!("{}", Self::row_not_found_message(row, self.size()));
        };
        let Some(tree) = row.get(col) else {
            panic!("{}", Self::tree_not_found_message(col, self.size()));
        };
        tree
    }

    fn get_mut_tree_at(&mut self, row: usize, col: usize) -> &mut Tree {
        let size = self.size();
        let Some(mut_row) = self.matrix.get_mut(row) else {
            panic!("{}", Self::row_not_found_message(row, size));
        };
        let Some(mut_tree) = mut_row.get_mut(col) else {
            panic!("{}", Self::tree_not_found_message(col, size));
        };
        mut_tree
    }

    fn row_not_found_message(row: usize, size: usize) -> String {
        format!("could not get row at index {row}! size of matrix is {size}")
    }

    fn tree_not_found_message(col: usize, size: usize) -> String {
        format!("could not get tree at index {col}! size of matrix is {size}")
    }

    fn size(&self) -> usize {
        self.matrix.len()
    }
}
