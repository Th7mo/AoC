use std::slice::Chunks;
use crate::get_union_of;
use crate::rucksack::Rucksack;

pub struct Group {
    pub first_rucksack: Rucksack,
    pub second_rucksack: Rucksack,
    pub third_rucksack: Rucksack,
}

impl Group {
    pub fn new(first_rucksack: Rucksack, second_rucksack: Rucksack, third_rucksack: Rucksack) -> Group {
       Group {
           first_rucksack,
           second_rucksack,
           third_rucksack,
       }
    }

    pub fn shared_item_in_rucksacks(self) -> char {
        let common_elements_first_and_second = get_union_of(
            &self.first_rucksack.get_all_items(),
            &self.second_rucksack.get_all_items()
        );
        let common_elements_first_and_second: String = common_elements_first_and_second.iter().collect();
        let common_element_of_all_three_rucksacks = get_union_of(
            &common_elements_first_and_second,
            &self.third_rucksack.get_all_items()
        );

        common_element_of_all_three_rucksacks[0]
    }

    pub fn convert_to_groups(rucksacks: Vec<Rucksack>) -> Vec<Group> {
        let group_chunks: Chunks<Rucksack> = rucksacks.chunks(3);
        let mut groups: Vec<Group> = Vec::new();

        for group in group_chunks {
            groups.push(
                Group::new(group[0].clone(), group[1].clone(), group[2].clone())
            );
        }
        groups
    }
}
