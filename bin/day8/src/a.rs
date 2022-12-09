use crate::forest_factory::ForestFactory;

pub fn solve() {
    let input_str = include_str!("input.txt");
    let mut forest = ForestFactory::create_forest(input_str);
    println!("{}", forest.visible_trees())
}
