use crate::get_rucksacks;
use crate::priority::Priority;

pub fn solve() {
    let rucksacks = get_rucksacks();
    let mut total_priority = 0;

    for rucksack in rucksacks {
        let char_in_both_compartments = rucksack.shared_item_in_compartments();
        total_priority += Priority::get_priority_of_item(char_in_both_compartments);
    }
    println!("{}", total_priority);
}
