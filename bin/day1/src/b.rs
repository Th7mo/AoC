use crate::file_parser;
use lib::file_reader;

pub fn solve() {
    let file = file_reader::file_in_lines(env!("CARGO_PKG_NAME"));
    let mut groups = file_parser::convert_to_groups(file.lines());
    println!("{}", calc_highest_elves(&mut groups));
}

fn calc_highest_elves(sums: &mut [i32]) -> i32 {
    sums.sort_unstable();
    let (_, highest_three) = sums.split_at(sums.len() - 3);
    highest_three.iter().sum()
}
