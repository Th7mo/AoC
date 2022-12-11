use crate::forest_factory::ForestFactory;

pub fn solve() -> u32 {
    let input_str = include_str!("input.txt");
    let mut forest = ForestFactory::create_forest(input_str);
    forest.visible_trees()
}
