use crate::cpu::Cpu;
use crate::instruction::Instruction;

pub fn solve() -> i32 {
    let file = include_str!("../res/input.txt");
    let mut cpu = Cpu::new(false);

    for line in file.lines() {
        cpu.execute(&Instruction::from(line));
    }

    cpu.signal_strength
}
