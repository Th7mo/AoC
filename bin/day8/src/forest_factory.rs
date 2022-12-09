use crate::forest::Forest;
use crate::tree::Tree;

pub struct ForestFactory;

impl ForestFactory {
    pub fn create_forest(input: &str) -> Forest {
        let mut forest = Forest::new();
        for row in input.lines() {
            Self::add_row(&mut forest, row)
        }

        forest
    }

    fn add_row(forest: &mut Forest, row: &str) {
        forest.add_row(Self::create_row(row));
    }

    fn create_row(line: &str) -> Vec<Tree> {
        line.chars().map(Tree::from).collect()
    }
}
