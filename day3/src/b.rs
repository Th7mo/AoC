use crate::file_reader;
use crate::group::Group;
use crate::priority::Priority;

pub fn solve() {
    let rucksacks = file_reader::get_rucksacks();
    let groups = Group::convert_to_groups(rucksacks);
    let mut total_priority = 0;

    for group in groups {
        let shared_item = group.shared_item_in_rucksacks();
        total_priority += Priority::get_priority_of_item(shared_item);
    }
    println!("{}", total_priority);
}
