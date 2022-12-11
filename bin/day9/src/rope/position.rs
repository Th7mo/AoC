#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn from(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn next_to(&self, other: &Position) -> bool {
        !self.x_too_far(other) && !self.y_too_far(other)
    }

    pub fn x_too_far(&self, other: &Position) -> bool {
        (self.x - other.x).abs() > 1
    }

    pub fn y_too_far(&self, other: &Position) -> bool {
        (self.y - other.y).abs() > 1
    }
}
