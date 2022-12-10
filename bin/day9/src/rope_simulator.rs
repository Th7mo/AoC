use crate::direction::Direction;
use crate::motion::Motion;
use crate::position::Position;
use crate::rope_history::RopeHistory;

pub struct RopeSimulator {
    history: RopeHistory,
    head: Position,
    tail: Position,
}

impl RopeSimulator {
    pub fn new() -> Self {
        let mut rope_history = RopeHistory::new();
        let start = Position::from(0, 0);
        rope_history.add_unique(start);
        RopeSimulator {
            history: rope_history,
            head: start,
            tail: start,
        }
    }

    pub fn execute(&mut self, motion: Motion) {
        for _ in 0..motion.amount_of_steps() {
            self.move_head(&motion);
            self.move_tail(&motion);
        }
    }

    fn move_head(&mut self, motion: &Motion) {
        match motion.direction {
            Direction::X => self.head.x += motion.get_step(),
            Direction::Y => self.head.y += motion.get_step(),
        }
    }

    fn move_tail(&mut self, motion: &Motion) {
        if self.tail.next_to(&self.head) {
            return;
        }

        match motion.direction {
            Direction::X => {
                self.tail.x += motion.get_step();
                self.tail.y += self.head.y - self.tail.y
            }
            Direction::Y => {
                self.tail.x += self.head.x - self.tail.x;
                self.tail.y += motion.get_step()
            }
        }
        self.history.add_unique(self.tail);
    }

    pub fn amount_of_visited_places(&self) -> usize {
        self.history.len()
    }
}
