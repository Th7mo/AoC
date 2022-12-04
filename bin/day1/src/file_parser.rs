pub fn convert_to_groups(array: Vec<&str>) -> Vec<i32> {
    let mut sums: Vec<i32> = Vec::new();
    let mut current_sum = 0;
    for value in array {
        if value.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += to_i32(value);
        }
    }

    sums
}

fn to_i32(str: &str) -> i32 {
    str.parse().unwrap()
}
