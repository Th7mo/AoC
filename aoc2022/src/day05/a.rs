use crate::day05::file_parser;

pub fn solve() {
    let (mut depot, instructions) = file_parser::get_depot_and_instructions();
    depot.execute_all(&instructions);
    println!("{}", depot.get_most_upper_crates_as_message());
}
