use crate::get_union_of;
use crate::rucksack::Rucksack;
use std::slice::Chunks;

pub struct Group {
    pub rucksack1: Rucksack,
    pub rucksack2: Rucksack,
    pub rucksack3: Rucksack,
}

impl Group {
    pub fn new(rucksack1: Rucksack, rucksack2: Rucksack, rucksack3: Rucksack) -> Group {
        Group {
            rucksack1,
            rucksack2,
            rucksack3,
        }
    }

    pub fn shared_item_in_rucksacks(&self) -> char {
        let common_elements_first_and_second = get_union_of(
            &self.rucksack1.get_all_items(),
            &self.rucksack2.get_all_items(),
        );
        let common_elements_first_and_second: String =
            common_elements_first_and_second.iter().collect();
        let common_element_of_all_three_rucksacks = get_union_of(
            &common_elements_first_and_second,
            &self.rucksack3.get_all_items(),
        );

        common_element_of_all_three_rucksacks[0]
    }

    pub fn convert_to_groups(rucksacks: Vec<Rucksack>) -> Vec<Group> {
        let group_chunks: Chunks<Rucksack> = rucksacks.chunks(3);
        let mut groups: Vec<Group> = Vec::new();

        for group in group_chunks {
            groups.push(Group::new(
                group[0].clone(),
                group[1].clone(),
                group[2].clone(),
            ));
        }
        groups
    }
}
