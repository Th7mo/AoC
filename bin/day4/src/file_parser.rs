use crate::assignment::Assignment;
use crate::pair::Pair;
use std::str::Lines;

pub fn get_assignment_pairs() -> Vec<Pair> {
    let file = include_str!("input.txt");
    parse_file(file.lines())
}

fn parse_file(lines: Lines) -> Vec<Pair> {
    let mut assignment_pairs: Vec<Pair> = Vec::new();

    for line in lines {
        let (left_assignment, right_assignment) = line
            .split_once(',')
            .expect("Input is not valid, missing ',' delimiter between tasks");
        let left_assignment = Assignment::new(left_assignment);
        let right_assignment = Assignment::new(right_assignment);
        assignment_pairs.push(Pair::new(left_assignment, right_assignment));
    }

    assignment_pairs
}
