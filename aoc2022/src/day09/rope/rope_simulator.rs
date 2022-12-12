use crate::day09::rope::Motion;
use crate::day09::rope::Rope;
use crate::day09::rope::RopeHistory;

pub struct RopeSimulator {
    history: RopeHistory,
    rope: Rope,
}

impl RopeSimulator {
    pub fn new(rope_length: u32) -> Self {
        let mut rope_history = RopeHistory::new();
        let rope = Rope::with_length(rope_length);
        rope_history.add_unique(*rope.tail());
        RopeSimulator {
            history: rope_history,
            rope,
        }
    }

    pub fn execute(&mut self, motion: Motion) {
        for _ in 0..motion.amount_of_steps() {
            self.rope.drag(&motion);
            self.history.add_unique(*self.rope.tail());
        }
    }

    pub fn amount_of_visited_places(&self) -> usize {
        self.history.len()
    }
}
