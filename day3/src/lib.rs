pub mod a;
pub mod b;
pub mod rucksack;
pub mod group;
pub mod priority;

use std::collections::HashSet;
use crate::rucksack::Rucksack;

pub fn get_rucksacks() -> Vec<Rucksack> {
    let file = read_file();
    parse_file(&file)
}

fn read_file() -> String {
    std::fs::read_to_string("./src/input.txt")
        .expect("Could not read file!")
}

fn parse_file(file: &str) -> Vec<Rucksack> {
    let rucksacks: Vec<&str> = file.trim().split("\r\n").collect();
    let mut rucksacks_with_compartments: Vec<Rucksack> = Vec::new();

    for line in rucksacks {
        let middle_index = line.len() / 2;
        let compartments = line.split_at(middle_index);
        let rucksack = Rucksack::new(compartments);
        rucksacks_with_compartments.push(rucksack);
    }

    rucksacks_with_compartments
}

pub fn get_union_of(first: &str, second: &str) -> Vec<char> {
    let mut known_elements: HashSet<char> = HashSet::new();
    let mut common_elements = Vec::new();

    for item in first.chars() {
        known_elements.insert(item);
    }

    for item in second.chars() {
        if known_elements.contains(&item) {
            common_elements.push(item);
        }
    }

    common_elements
}
