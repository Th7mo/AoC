use crate::day10::Crt;
use crate::day10::Instruction;

pub struct Cpu {
    pub signal_strength: i32,
    draw: bool,
    register_x: i32,
    cycle_count: i32,
}

impl Cpu {
    pub fn new(draw: bool) -> Self {
        Cpu {
            signal_strength: 0,
            draw,
            register_x: 1,
            cycle_count: 0,
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.clock_length() {
            self.increase_cycle_count();
            self.update_signal_strength();
        }
        match instruction {
            Instruction::Addx(value) => self.addx(value),
            Instruction::Noop => {}
        }
    }

    fn addx(&mut self, value: &i32) {
        self.update_register(*value);
    }

    fn increase_cycle_count(&mut self) {
        if self.draw {
            Crt::draw_pixel(self.cycle_count, self.register_x);
        }
        self.cycle_count += 1;
    }

    fn update_register(&mut self, value: i32) {
        self.register_x += value;
    }

    fn update_signal_strength(&mut self) {
        if (self.cycle_count + 20) % 40 == 0 {
            self.signal_strength += self.cycle_count * self.register_x;
        }
    }
}
