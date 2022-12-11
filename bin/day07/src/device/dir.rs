pub struct Dir {
    size: u32,
}

impl Dir {
    pub fn new() -> Self {
        Dir { size: 0 }
    }

    pub fn add(&mut self, value: u32) {
        self.size += value;
    }

    pub fn smaller_than(&self, threshold: u32) -> bool {
        self.size < threshold
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}
