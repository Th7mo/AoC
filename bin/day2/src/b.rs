use day2::parse_file;
use lib::file_reader;

pub fn solve() {
    let file = file_reader::file_in_lines(env!("CARGO_PKG_NAME"));
    let rounds = parse_file(file.lines());
    let mut total_score = 0;

    for round in rounds {
        let (raw_opponent_game_move, raw_desired_game_result) = round;
        let opponent_game_move = map_to_game_move(raw_opponent_game_move);
        let desired_game_result = map_to_desired_game_result(raw_desired_game_result);
        let your_game_move = calculate_your_move(&desired_game_result, &opponent_game_move);
        total_score += calculate_score(desired_game_result, your_game_move);
    }

    println!("{}", total_score);
}

fn calculate_your_move(desired_game_result: &GameResult, opponent_move: &GameMove) -> GameMove {
    match opponent_move {
        GameMove::Rock => match desired_game_result {
            GameResult::Won => GameMove::Paper,
            GameResult::Equal => GameMove::Rock,
            GameResult::Lost => GameMove::Scissors,
        },
        GameMove::Paper => match desired_game_result {
            GameResult::Lost => GameMove::Rock,
            GameResult::Equal => GameMove::Paper,
            GameResult::Won => GameMove::Scissors,
        },
        GameMove::Scissors => match desired_game_result {
            GameResult::Won => GameMove::Rock,
            GameResult::Lost => GameMove::Paper,
            GameResult::Equal => GameMove::Scissors,
        },
    }
}

fn map_to_game_move(raw_game_move: &str) -> GameMove {
    match raw_game_move {
        "A" => GameMove::Rock,
        "B" => GameMove::Paper,
        "C" => GameMove::Scissors,
        _ => unreachable!("The input of opponent is always in (A,B,C)"),
    }
}

fn map_to_desired_game_result(raw_game_move: &str) -> GameResult {
    match raw_game_move {
        "X" => GameResult::Lost,
        "Y" => GameResult::Equal,
        "Z" => GameResult::Won,
        _ => unreachable!("The input for you is always in (X,Y,Z)"),
    }
}

fn calculate_score(game_result: GameResult, game_move: GameMove) -> i32 {
    game_result as i32 + game_move as i32
}

enum GameMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum GameResult {
    Won = 6,
    Equal = 3,
    Lost = 0,
}
