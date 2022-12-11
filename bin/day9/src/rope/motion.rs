#[derive(Copy, Clone)]
pub struct Motion {
    pub direction: Direction,
    amount: i32,
}

impl Motion {
    pub fn from(raw_motion: &str) -> Self {
        let Some((direction, amount)) = raw_motion.split_once(' ') else {
            panic!("{raw_motion} is not a valid motion");
        };
        let mut amount: i32 = amount.parse().unwrap();
        match raw_motion.chars().next() {
            Some('L') | Some('D') => amount = -amount,
            _ => {}
        }

        Motion {
            direction: Direction::from(direction),
            amount,
        }
    }

    pub fn get_step(&self) -> i32 {
        self.amount / self.amount.abs()
    }

    pub fn amount_of_steps(&self) -> i32 {
        self.amount.abs()
    }
}

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
