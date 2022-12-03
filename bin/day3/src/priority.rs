const POSSIBLE_ITEMS: &str ="abcdefghijklmnopqrstuvwxyz\
                             ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Priority {}

impl Priority {
    pub fn get_priority_of_item(char_in_all_rucksacks: char) -> usize {
        POSSIBLE_ITEMS.find(char_in_all_rucksacks)
            .expect("could not find character in alphabet") + 1
    }
}
