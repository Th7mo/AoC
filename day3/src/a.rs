use std::collections::HashSet;

use day3::read_file;
use day3::parse_file;

const ITEMS: &str ="abcdefghijklmnopqrstuvwxyz\
                    ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn solve() {
    let input_file = read_file();
    let rucksacks = parse_file(&input_file);
    let mut total_priority = 0;
    for rucksack in rucksacks {
        let char_in_both_compartments =find_item_in_both_compartments(rucksack);
        total_priority += ITEMS.find(char_in_both_compartments)
            .expect("could not find character in alphabet") + 1;
    }
    println!("{}", total_priority);
}

fn find_item_in_both_compartments(compartments: (&str, &str)) -> char {
    let mut known_elements: HashSet<char> = HashSet::new();

    let (first_compartment, second_compartment) = compartments;

    for item in first_compartment.chars() {
        known_elements.insert(item);
    }

    for item in second_compartment.chars() {
        if known_elements.contains(&item) {
            return item;
        }
    }

    unreachable!("Could not find character that is in both compartments!");
}
