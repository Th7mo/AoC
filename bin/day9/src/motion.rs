use crate::direction::Direction;
use std::ops::Neg;

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
            Some('L') | Some('D') => amount = amount.neg(),
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
