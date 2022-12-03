use crate::rucksack::Rucksack;

pub fn get_rucksacks() -> Vec<Rucksack> {
    let file = file_reader::read_file(env!("CARGO_PKG_NAME"));
    parse_file(&file)
}

fn parse_file(file: &str) -> Vec<Rucksack> {
    let rucksacks: Vec<&str> = file.trim().split("\r\n").collect();
    let mut rucksacks_with_compartments: Vec<Rucksack> = Vec::new();

    for line in rucksacks {
        let middle_index = line.len() / 2;
        let compartments = line.split_at(middle_index);
        let rucksack = Rucksack::new(compartments);
        rucksacks_with_compartments.push(rucksack);
    }

    rucksacks_with_compartments
}
