use std::collections::HashSet;

const POSSIBLE_ITEMS: &str ="abcdefghijklmnopqrstuvwxyz\
                    ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Rucksack {
    pub first_compartment: String,
    pub second_compartment: String,
}

impl Rucksack {
    pub fn new(compartments: (&str, &str)) -> Rucksack {
        Rucksack {
            first_compartment: compartments.0.to_string(),
            second_compartment: compartments.1.to_string(),
        }
    }

    pub fn get_priority_of_item(char_in_both_compartments: char) -> usize {
        POSSIBLE_ITEMS.find(char_in_both_compartments)
            .expect("could not find character in alphabet") + 1
    }

    pub fn shared_item_in_compartments(self) -> char {
        let mut known_elements: HashSet<char> = HashSet::new();

        for item in self.first_compartment.chars() {
            known_elements.insert(item);
        }

        for item in self.second_compartment.chars() {
            if known_elements.contains(&item) {
                return item;
            }
        }

        unreachable!("Could not find character that is in both compartments!");
    }
}


