use crate::file_parser;

pub fn solve() {
    let file = include_str!("../res/input.txt");
    let mut groups = file_parser::convert_to_groups(file.lines());
    println!("{}", calc_highest_elves(&mut groups));
}

fn calc_highest_elves(sums: &mut [i32]) -> i32 {
    sums.sort_unstable();
    let (_, highest_three) = sums.split_at(sums.len() - 3);
    highest_three.iter().sum()
}
