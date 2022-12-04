pub fn convert_to_groups(array: Vec<&str>) -> Vec<i32> {
    let mut sums: Vec<i32> = Vec::new();
    let mut current_sum = 0;

    for value in array {
        if value.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += i32_of(value);
        }
    }

    sums
}

fn i32_of(str: &str) -> i32 {
    str.parse()
        .unwrap_or_else(|_| panic!("Can't convert {str} to a number"))
}
