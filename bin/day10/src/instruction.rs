pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    pub fn from(raw_instruction: &str) -> Self {
        match raw_instruction.chars().next().unwrap() {
            'a' => Instruction::Addx(Instruction::get_value_from_raw_instruction(raw_instruction)),
            _ => Instruction::Noop,
        }
    }

    fn get_value_from_raw_instruction(raw_instruction: &str) -> i32 {
        let value_as_str = raw_instruction
            .split_once(' ')
            .expect("could not find ' ' delimiter")
            .1;
        value_as_str.parse().unwrap()
    }

    pub fn clock_length(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}
