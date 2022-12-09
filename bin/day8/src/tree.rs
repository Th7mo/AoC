pub struct Tree {
    pub height: u8,
    pub visible: bool,
    pub scenic_score: u32,
}

impl Tree {
    pub fn from(height: char) -> Self {
        Tree {
            height: Self::u8_of(height),
            visible: false,
            scenic_score: 0,
        }
    }

    fn u8_of(height: char) -> u8 {
        height as u8 - 48
    }

    pub fn higher_than(&self, height: u8) -> bool {
        self.height > height
    }

    pub fn smaller_or_equal_to(&self, other: &Self) -> bool {
        self.height <= other.height
    }
}
