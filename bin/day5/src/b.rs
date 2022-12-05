use crate::file_parser;

pub fn solve() {
    let (mut depot, instructions) = file_parser::get_depot_and_instructions();
    depot.execute_all_with_all_crates_lifted_together(&instructions);
    println!("{}", depot.get_most_upper_crates_as_message());
}
