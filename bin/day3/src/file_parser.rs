use crate::rucksack::Rucksack;
use lib::file_reader;
use std::str::Lines;

pub fn get_rucksacks() -> Vec<Rucksack> {
    let file = file_reader::file_in_lines(env!("CARGO_PKG_NAME"));
    parse_file(file.lines())
}

fn parse_file(lines: Lines) -> Vec<Rucksack> {
    let mut rucksacks_with_compartments: Vec<Rucksack> = Vec::new();

    for line in lines {
        let middle_index = line.len() / 2;
        let compartments = line.split_at(middle_index);
        let rucksack = Rucksack::new(&compartments);
        rucksacks_with_compartments.push(rucksack);
    }

    rucksacks_with_compartments
}
