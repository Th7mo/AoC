use crate::file_parser;

pub fn solve() {
    let pairs = file_parser::get_assignment_pairs();
    let mut total_pairs_that_have_double_assignments = 0;

    for pair in pairs {
        if pair.assignment_fully_contains_other() {
            total_pairs_that_have_double_assignments += 1;
        }
    }

    println!("{}", total_pairs_that_have_double_assignments);
}
