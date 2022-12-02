pub fn solve() {
    let file = read_file();
    let round_choices = parse_file(&file);
    let mut total_points_rewarded = 0;

    for round in round_choices {
        let (game, choice) = validate_game(round);
        total_points_rewarded += calculate_points(game, choice);
    }

    println!("{}", total_points_rewarded);
}

fn read_file() -> String {
    std::fs::read_to_string("./src/input.txt")
        .expect("Could not read file.")
}

fn parse_file(file: &str) -> Vec<(&str, &str)> {
    let file_lines: Vec<&str> = file.trim().split("\r\n").collect();
    let mut round_choices: Vec<(&str, &str)> = Vec::new();

    for file_line in file_lines {
        let round_choice = file_line.split_once(' ')
            .expect("Input file not correct");
        round_choices.push(round_choice);
    }

    round_choices
}

fn validate_game(choices: (&str, &str)) -> (Game, Choice) {
    let (opponent_choice, your_choice) = map_to_choices(choices);
    let game_end = match opponent_choice {
        Choice::Rock => match your_choice {
            Choice::Rock => Game::Equal,
            Choice::Paper => Game::Won,
            Choice::Scissors => Game::Lost,
        },
        Choice::Paper => match your_choice {
            Choice::Rock => Game::Lost,
            Choice::Paper => Game::Equal,
            Choice::Scissors => Game::Won,
        },
        Choice::Scissors => match your_choice {
            Choice::Rock => Game::Won,
            Choice::Paper => Game::Lost,
            Choice::Scissors => Game::Equal,
        },
    };

    (game_end, your_choice)
}

fn map_to_choices(raw_choices: (&str, &str)) -> (Choice, Choice) {
    let (raw_opponent_choice, raw_your_choice) = raw_choices;
    let opponent_choice = map_choice(raw_opponent_choice);
    let your_choice = map_choice(raw_your_choice);
    (opponent_choice, your_choice)
}

fn map_choice(raw_choice: &str) -> Choice {
    if raw_choice == "A" || raw_choice == "X" {
        return Choice::Rock;
    } else if raw_choice == "B" || raw_choice == "Y" {
        return Choice::Paper;
    } else if raw_choice == "C" || raw_choice == "Z" {
        return Choice::Scissors;
    }

    panic!()
}

fn calculate_points(game: Game, choice: Choice) -> i32 {
    game as i32 + choice as i32
}

enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Game {
    Won = 6,
    Equal = 3,
    Lost = 0
}
