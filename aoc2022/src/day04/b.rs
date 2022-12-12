use crate::day04::file_parser;

pub fn solve() {
    let pairs = file_parser::get_assignment_pairs();
    let mut total_pairs_to_have_overlapping_range = 0;

    for pair in pairs {
        if pair.assignments_have_shared_ids() {
            total_pairs_to_have_overlapping_range += 1;
        }
    }

    println!("{}", total_pairs_to_have_overlapping_range);
}
