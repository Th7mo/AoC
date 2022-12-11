use crate::cpu::Cpu;
use crate::instruction::Instruction;

pub fn solve() {
    let file = include_str!("input.txt");
    let mut cpu = Cpu::new();

    for line in file.lines() {
        cpu.draw(&Instruction::from(line));
    }
}
