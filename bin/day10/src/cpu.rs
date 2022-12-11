use crate::instruction::Instruction;

pub struct Cpu {
    register_x: i32,
    cycle_count: i32,
    pub signal_strength: i32,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register_x: 1,
            cycle_count: 0,
            signal_strength: 0,
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        self.eval();
        match instruction {
            Instruction::Noop => {}
            Instruction::Addx(value) => {
                self.eval();
                self.update_register(*value);
            }
        }
    }

    fn update_register(&mut self, value: i32) {
        self.register_x += value;
    }

    fn eval(&mut self) {
        self.cycle_count += 1;
        if (self.cycle_count + 20) % 40 == 0 {
            println!("{} {}", self.cycle_count, self.register_x);
            self.signal_strength += self.cycle_count * self.register_x;
        }
    }
}
