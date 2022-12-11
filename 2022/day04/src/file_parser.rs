use crate::pair::Pair;

pub fn get_assignment_pairs() -> Vec<Pair> {
    let file = include_str!("../res/input.txt");
    file.lines().map(Pair::new).collect()
}
