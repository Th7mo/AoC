pub struct Instruction {
    pub move_count: u8,
    pub source_stack: usize,
    pub target_stack: usize,
}

impl Instruction {
    pub fn from(line: &str) -> Self {
        let line = line.replace("move ", "");
        let Some((move_count, remainder)) = line.split_once(" from ") else {
            panic!("incorrect instruction set! Make sure ' from ' keyword is in 01.txt");
        };

        let Some((source_stack, target_stack)) = remainder.split_once(" to ") else {
            panic!("incorrect instruction set! Make sure ' to ' keyword is in 01.txt");
        };

        Instruction {
            move_count: move_count.parse().expect("could not convert to number"),
            source_stack: Instruction::usize_of(source_stack) - 1,
            target_stack: Instruction::usize_of(target_stack) - 1,
        }
    }

    fn usize_of(str: &str) -> usize {
        str.parse().expect("could not convert to number")
    }
}
