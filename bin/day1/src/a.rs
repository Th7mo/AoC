use crate::file_parser;
use lib::file_reader;

pub fn solve() {
    let file = file_reader::file_in_lines(env!("CARGO_PKG_NAME"));
    let groups = file_parser::convert_to_groups(file.lines());
    println!("{}", calc_highest_elf(&groups));
}

fn calc_highest_elf(sums: &[i32]) -> i32 {
    *sums.iter().max().expect("array was probably empty")
}
