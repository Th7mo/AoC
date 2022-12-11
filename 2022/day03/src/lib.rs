pub mod a;
pub mod b;
mod file_parser;
mod group;
mod priority;
mod rucksack;

use std::collections::HashSet;

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
