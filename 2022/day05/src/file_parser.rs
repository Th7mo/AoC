use crate::depot::Depot;
use crate::instruction::Instruction;

pub fn get_depot_and_instructions() -> (Depot, Vec<Instruction>) {
    let file = include_str!("../res/input.txt");
    let delimiter = " 1   2   3   4   5   6   7   8   9 ";
    let Some((depot_lines, instruction_lines)) = file.split_once(delimiter) else {
        panic!("can't find 1..=9 delimiter between depot and move instructions in input file");
    };

    (get_depot(depot_lines), get_instructions(instruction_lines))
}

fn get_depot(depot_str: &str) -> Depot {
    Depot::from(depot_str)
}

fn get_instructions(instruction_str: &str) -> Vec<Instruction> {
    instruction_str
        .trim_start()
        .lines()
        .map(Instruction::from)
        .collect()
}
