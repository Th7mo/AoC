pub fn parse_file(file: &str) -> Vec<(&str, &str)> {
    let file_lines: Vec<&str> = file.trim().split("\r\n").collect();
    let mut round_choices: Vec<(&str, &str)> = Vec::new();

    for file_line in file_lines {
        let round_choice = file_line.split_once(' ').expect("Input file not correct");
        round_choices.push(round_choice);
    }

    round_choices
}
