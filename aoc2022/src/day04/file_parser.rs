use crate::day04::Pair;

pub fn get_assignment_pairs() -> Vec<Pair> {
    let file = include_str!("../../res/04.txt");
    file.lines().map(Pair::new).collect()
}
