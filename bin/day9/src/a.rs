use crate::motion::Motion;
use crate::rope_simulator::RopeSimulator;

pub fn solve() -> usize {
    let file = include_str!("input.txt");
    let mut rope_simulator = RopeSimulator::new();

    for line in file.lines() {
        let motion = Motion::from(line);
        rope_simulator.execute(motion);
    }
    rope_simulator.amount_of_visited_places()
}