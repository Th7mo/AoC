pub struct Assignment {
    pub first: i32,
    pub last: i32,
}

impl Assignment {
    pub fn new(instruction: &str) -> Self {
        let (first, last) = Self::map_to_i32s(instruction);
        Assignment { first, last }
    }

    fn map_to_i32s(instruction: &str) -> (i32, i32) {
        let Some((first, last)) = instruction.split_once('-') else {
            panic!("Missing '-' delimiter in {instruction}")
        };
    
        (first.parse().unwrap(), last.parse().unwrap())
    }
}
