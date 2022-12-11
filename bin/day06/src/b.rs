use crate::message_decoder;

pub fn solve() {
    let message = include_str!("input.txt").trim();
    message_decoder::print_first_n_unique_character(message, 14);
}
