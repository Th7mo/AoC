pub fn solve() {
    let file = file_reader::read_file(env!("CARGO_PKG_NAME"));
    let lines: Vec<&str> = file.split("\r\n").collect();
    let mut groups = convert_to_groups(lines);
    println!("{}", calc_highest_elfs(&mut groups));
}

fn convert_to_groups(array: Vec<&str>) -> Vec<i32> {
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

fn calc_highest_elfs(sums: &mut [i32]) -> i32 {
    sums.sort();
    let (_, highest_three) = sums.split_at(sums.len() - 3);
    highest_three.iter().sum()
}

fn to_i32(str: &str) -> i32 {
    str.parse().unwrap()
}
