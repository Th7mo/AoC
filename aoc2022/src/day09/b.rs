use crate::day09::rope::{Motion, RopeSimulator};

pub fn solve() -> usize {
    let file = include_str!("../../res/09.txt");
    let mut rope_simulator = RopeSimulator::new(10);

    for line in file.lines() {
        let motion = Motion::from(line);
        rope_simulator.execute(motion);
    }
    rope_simulator.amount_of_visited_places()
}
