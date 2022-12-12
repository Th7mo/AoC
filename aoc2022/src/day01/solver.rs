use crate::day01::file_parser;
use crate::Solvable;

pub struct Solver;

impl Solver {
    fn get_groups() -> Vec<i32> {
        let file = include_str!("../../res/01.txt");
        file_parser::convert_to_groups(file.lines())
    }

    fn calc_highest_elf(sums: &[i32]) -> i32 {
        *sums.iter().max().expect("array was probably empty")
    }

    fn calc_highest_elves(sums: &mut [i32]) -> i32 {
        sums.sort_unstable();
        let (_, highest_three) = sums.split_at(sums.len() - 3);
        highest_three.iter().sum()
    }
}

impl Solvable for Solver {
    fn solve_part_1() {
        let groups = Self::get_groups();
        println!("{}", Self::calc_highest_elf(&groups));
    }

    fn solve_part_2() {
        let mut groups = Self::get_groups();
        println!("{}", Self::calc_highest_elves(&mut groups));
    }
}
