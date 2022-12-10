use crate::direction::Direction;
use crate::motion::Motion;
use crate::position::Position;

pub struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    pub fn with_length(size: u32) -> Self {
        let start = Position::from(0, 0);
        let mut knots = Vec::new();

        for _ in 0..size {
            knots.push(start);
        }
        Rope { knots }
    }

    pub fn drag(&mut self, motion: &Motion) {
        self.move_head(motion);
        self.update_remainder();
    }

    fn move_head(&mut self, motion: &Motion) {
        match motion.direction {
            Direction::X => self.mut_head().x += motion.get_step(),
            Direction::Y => self.mut_head().y += motion.get_step(),
        }
    }

    fn update_remainder(&mut self) {
        for knot in 0..self.knots.len() - 1 {
            self.move_following_knot(knot);
        }
    }

    fn move_following_knot(&mut self, head_index: usize) {
        let head = *self.get_at(head_index);
        let following_knot = self.get_mut_at(head_index + 1);

        if following_knot.next_to(&head) {
            return;
        }

        following_knot.x += Rope::get_diff_step(head.x, following_knot.x);
        following_knot.y += Rope::get_diff_step(head.y, following_knot.y);
    }

    fn get_diff_step(x: i32, y: i32) -> i32 {
        let diff = x - y;
        match diff {
            d if d > 0 => 1,
            d if d < 0 => -1,
            _ => 0,
        }
    }

    fn mut_head(&mut self) -> &mut Position {
        self.get_mut_at(0)
    }

    pub fn tail(&self) -> &Position {
        let len = self.knots.len();
        self.get_at(len - 1)
    }

    fn get_at(&self, index: usize) -> &Position {
        self.knots.get(index).unwrap()
    }

    fn get_mut_at(&mut self, index: usize) -> &mut Position {
        self.knots.get_mut(index).unwrap()
    }
}
