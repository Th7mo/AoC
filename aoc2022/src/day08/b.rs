use crate::day08::forest::ForestFactory;

pub fn solve() -> u32 {
    let input_str = include_str!("../../res/08.txt");
    let mut forest = ForestFactory::create_forest(input_str);
    forest.highest_scenic_score()
}
