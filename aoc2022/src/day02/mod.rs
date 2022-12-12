use std::str::Lines;

pub mod a;
pub mod b;

pub fn parse_file(lines: Lines) -> Vec<(&str, &str)> {
    let mut round_choices: Vec<(&str, &str)> = Vec::new();

    for file_line in lines {
        let round_choice = file_line.split_once(' ').expect("Input file not correct");
        round_choices.push(round_choice);
    }

    round_choices
}
