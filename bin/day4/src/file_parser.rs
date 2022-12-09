use crate::pair::Pair;

pub fn get_assignment_pairs() -> Vec<Pair> {
    let file = include_str!("input.txt");
    file.lines().map(Pair::new).collect()
}
