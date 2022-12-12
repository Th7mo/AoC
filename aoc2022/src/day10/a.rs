use crate::day10::Cpu;
use crate::day10::Instruction;

pub fn solve() -> i32 {
    let file = include_str!("../../res/10.txt");
    let mut cpu = Cpu::new(false);

    for line in file.lines() {
        cpu.execute(&Instruction::from(line));
    }

    cpu.signal_strength
}
