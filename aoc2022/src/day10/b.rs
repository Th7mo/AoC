use crate::day10::Cpu;
use crate::day10::Instruction;

pub fn solve() {
    let file = include_str!("../../res/10.txt");
    let mut cpu = Cpu::new(true);

    for line in file.lines() {
        cpu.execute(&Instruction::from(line));
    }
}
