#[derive(Copy, Clone)]
pub enum Direction {
    X,
    Y,
}

impl Direction {
    pub fn from(raw_direction: &str) -> Self {
        match raw_direction {
            "L" | "R" => Direction::X,
            "U" | "D" => Direction::Y,
            _ => panic!("'{raw_direction}' is not a valid direction: [L, R, U, D]"),
        }
    }
}
