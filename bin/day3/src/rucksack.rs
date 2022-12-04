use crate::get_union_of;

#[derive(Clone)]
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

    pub fn shared_item_in_compartments(&self) -> char {
        get_union_of(&self.first_compartment, &self.second_compartment)[0]
    }

    pub fn get_all_items(&self) -> String {
        format!("{}{}", self.first_compartment, self.second_compartment)
    }
}
