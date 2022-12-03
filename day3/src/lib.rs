pub fn read_file() -> String {
    std::fs::read_to_string("./src/input.txt")
        .expect("Could not read file!")
}

pub fn parse_file(file: &str) -> Vec<(&str, &str)> {
    let rucksacks: Vec<&str> = file.trim().split("\r\n").collect();
    let mut rucksacks_with_compartments: Vec<(&str, &str)> = Vec::new();

    for line in rucksacks {
        let middle_index = line.len() / 2;
        let halves = line.split_at(middle_index);
        rucksacks_with_compartments.push(halves);
    }

    rucksacks_with_compartments
}
