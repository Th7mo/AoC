use crate::assignment::Assignment;

pub struct Pair {
    pub left: Assignment,
    pub right: Assignment,
}

impl Pair {
    pub fn new(assignments: &str) -> Self {
        let (left_assignment, right_assignment) = assignments
            .split_once(',')
            .expect("Input is not valid, missing ',' delimiter between tasks");
        Pair {
            left: Assignment::new(left_assignment),
            right: Assignment::new(right_assignment),
        }
    }

    pub fn assignment_fully_contains_other(&self) -> bool {
        (self.left.first >= self.right.first && self.left.last <= self.right.last)
            || (self.right.first >= self.left.first && self.right.last <= self.left.last)
    }

    pub fn assignments_have_shared_ids(&self) -> bool {
        Self::is_in_range(&self.left, &self.right) || Self::is_in_range(&self.right, &self.left)
    }

    fn is_in_range(left: &Assignment, right: &Assignment) -> bool {
        (left.first >= right.first && left.first <= right.last)
            || (left.last >= right.first && left.last <= right.last)
    }
}
