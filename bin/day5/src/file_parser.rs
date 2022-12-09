use crate::depot::Depot;
use crate::instruction::Instruction;

pub fn get_depot_and_instructions() -> (Depot, Vec<Instruction>) {
    let file = include_str!("input.txt");
    let delimiter = " 1   2   3   4   5   6   7   8   9 ";
    let Some((depot_lines, instruction_lines)) = file.split_once(delimiter) else {
        panic!("incorrect input file, missing newline and 1..=9 count");
    };

    (
        get_depot(depot_lines.trim_end()),
        get_instructions(instruction_lines.trim_start()),
    )
}

fn get_depot(depot_str: &str) -> Depot {
    Depot::new(depot_str)
}

fn get_instructions(instruction_str: &str) -> Vec<Instruction> {
    instruction_str.lines().map(Instruction::new).collect()
}
