pub struct Tree {
    pub height: u8,
    pub visible: bool,
}

impl Tree {
    pub fn from(height: char) -> Self {
        Tree {
            height: Self::u8_of(height),
            visible: false,
        }
    }

    fn u8_of(height: char) -> u8 {
        height as u8 - 48
    }

    pub fn higher_than(&self, height: u8) -> bool {
        self.height > height
    }
}
