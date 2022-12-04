use crate::assignment::Assignment;
use crate::pair::Pair;

pub fn get_assignment_pairs() -> Vec<Pair> {
    let file = file_reader::read_file(env!("CARGO_PKG_NAME"));
    parse_file(&file)
}

fn parse_file(file: &str) -> Vec<Pair> {
    let lines: Vec<&str> = file.trim().split("\r\n").collect();
    let mut assignment_pairs: Vec<Pair> = Vec::new();

    for line in lines {
        let (left_assignment, right_assignment) = line.split_once(',')
            .expect("Input is not valid, missing ',' delimiter between tasks");
        let left_assignment = Assignment::new(left_assignment);
        let right_assignment = Assignment::new(right_assignment);
        assignment_pairs.push(Pair::new(left_assignment, right_assignment));
    }

    assignment_pairs
}
