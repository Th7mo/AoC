use crate::day02::parse_file;

pub fn solve() {
    let file = include_str!("../../res/02.txt");
    let rounds = parse_file(file.lines());
    let mut total_score = 0;

    for round in rounds {
        let (opponent_move, your_move) = map_to_round_moves(round);
        let game_result = decide_game_result(&opponent_move, &your_move);
        total_score += calculate_score_of_round(game_result, your_move);
    }

    println!("{}", total_score);
}

fn decide_game_result(opponent_move: &GameMove, your_move: &GameMove) -> GameResult {
    match opponent_move {
        GameMove::Rock => match your_move {
            GameMove::Rock => GameResult::Equal,
            GameMove::Paper => GameResult::Won,
            GameMove::Scissors => GameResult::Lost,
        },
        GameMove::Paper => match your_move {
            GameMove::Rock => GameResult::Lost,
            GameMove::Paper => GameResult::Equal,
            GameMove::Scissors => GameResult::Won,
        },
        GameMove::Scissors => match your_move {
            GameMove::Rock => GameResult::Won,
            GameMove::Paper => GameResult::Lost,
            GameMove::Scissors => GameResult::Equal,
        },
    }
}

fn map_to_round_moves(raw_game_moves: (&str, &str)) -> (GameMove, GameMove) {
    let (raw_opponent_move, raw_your_move) = raw_game_moves;
    let opponent_move = map_move_to_enum(raw_opponent_move);
    let your_move = map_move_to_enum(raw_your_move);
    (opponent_move, your_move)
}

fn map_move_to_enum(raw_move: &str) -> GameMove {
    match raw_move {
        "A" | "X" => GameMove::Rock,
        "B" | "Y" => GameMove::Paper,
        "C" | "Z" => GameMove::Scissors,
        _ => unreachable!("The input is always in (A,B,C) or (X,Y,Z)"),
    }
}

fn calculate_score_of_round(game_result: GameResult, game_move: GameMove) -> i32 {
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
