use crate::cpu::Cpu;
use crate::instruction::Instruction;

pub fn solve() -> i32 {
    let file = include_str!("input.txt");
    let mut cpu = Cpu::new();

    for line in file.lines() {
        cpu.execute(&Instruction::from(line));
    }

    cpu.signal_strength
}
