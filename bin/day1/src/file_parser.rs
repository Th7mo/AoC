use crate::i32_of;
use std::str::Lines;

pub fn convert_to_groups(lines: Lines) -> Vec<i32> {
    let mut sums: Vec<i32> = Vec::new();
    let mut current_sum = 0;

    for value in lines {
        if value.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += i32_of(value);
        }
    }

    sums
}
