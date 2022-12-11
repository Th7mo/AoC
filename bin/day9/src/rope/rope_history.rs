use crate::rope::Position;
use std::collections::HashSet;

pub struct RopeHistory {
    visited: HashSet<Position>,
}

impl RopeHistory {
    pub fn new() -> Self {
        RopeHistory {
            visited: HashSet::new(),
        }
    }

    pub fn add_unique(&mut self, position: Position) {
        self.visited.insert(position);
    }

    pub fn len(&self) -> usize {
        self.visited.len()
    }
}
