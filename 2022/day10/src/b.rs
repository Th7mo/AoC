use crate::cpu::Cpu;
use crate::instruction::Instruction;

pub fn solve() {
    let file = include_str!("../res/input.txt");
    let mut cpu = Cpu::new(true);

    for line in file.lines() {
        cpu.execute(&Instruction::from(line));
    }
}
