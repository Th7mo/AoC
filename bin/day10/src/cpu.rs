pub struct Cpu {
    register_x: i32,
    cycle_count: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register_x: 1,
            cycle_count: 0,
        }
    }
}
