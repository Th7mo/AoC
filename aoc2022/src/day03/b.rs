use crate::day03::file_parser;
use crate::day03::Group;
use crate::day03::Priority;

pub fn solve() {
    let rucksacks = file_parser::get_rucksacks();
    let groups = Group::convert_to_groups(&rucksacks);
    let mut total_priority = 0;

    for group in groups {
        let shared_item = group.shared_item_in_rucksacks();
        total_priority += Priority::get_priority_of_item(shared_item);
    }
    println!("{}", total_priority);
}
