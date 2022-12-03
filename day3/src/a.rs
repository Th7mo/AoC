use crate::file_parser;
use crate::priority::Priority;

pub fn solve() {
    let rucksacks = file_parser::get_rucksacks();
    let mut total_priority = 0;

    for rucksack in rucksacks {
        let char_in_both_compartments = rucksack.shared_item_in_compartments();
        total_priority += Priority::get_priority_of_item(char_in_both_compartments);
    }
    println!("{}", total_priority);
}
