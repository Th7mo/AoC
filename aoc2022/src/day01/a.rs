use crate::day01::file_parser;

pub fn solve() {
    let file = include_str!("../../res/01.txt");
    let groups = file_parser::convert_to_groups(file.lines());
    println!("{}", calc_highest_elf(&groups));
}

fn calc_highest_elf(sums: &[i32]) -> i32 {
    *sums.iter().max().expect("array was probably empty")
}
