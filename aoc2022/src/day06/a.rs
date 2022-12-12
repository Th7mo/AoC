use crate::day06::message_decoder;

pub fn solve() {
    let message = include_str!("../../res/06.txt").trim();
    message_decoder::print_first_n_unique_character(message, 4);
}
