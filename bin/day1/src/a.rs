use crate::file_parser;

pub fn solve() {
    let file = file_reader::read_file(env!("CARGO_PKG_NAME"));
    let lines: Vec<&str> = file.split("\r\n").collect();
    let groups = file_parser::convert_to_groups(lines);
    println!("{}", calc_highest_elf(&groups));
}

fn calc_highest_elf(sums: &[i32]) -> i32 {
    *sums.iter().max().expect("array was probably empty")
}
