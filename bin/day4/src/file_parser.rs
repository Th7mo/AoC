use crate::pair::Pair;

pub fn get_assignment_pairs() -> Vec<Pair> {
    let file = include_str!("input.txt");
    parse_file(Lines::from(file))
}

fn parse_file(lines: Lines) -> Vec<Pair> {
    let mut assignment_pairs: Vec<Pair> = Vec::new();

    for line in lines.lines {
        assignment_pairs.push(Pair::new(line));
    }
    assignment_pairs
}

struct Lines<'a> {
    pub lines: Vec<&'a str>,
}

impl<'a> Lines<'a> {
    pub fn from(input: &'a str) -> Self {
        Lines {
            lines: input.trim().split("\r\n").collect(),
        }
    }
}
