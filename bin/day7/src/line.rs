pub enum Line<'a> {
    Cd(&'a str),
    Ls(&'a str),
    ListFile(&'a str),
    ListDir(&'a str),
}

impl<'a> Line<'a> {
    pub fn from(line: &'a str) -> Self {
        match line.get(..=3).unwrap() {
            "$ cd" => Self::Cd(line),
            "$ ls" => Self::Ls(line),
            "dir " => Self::ListDir(line),
            _ => Self::ListFile(line),
        }
    }
}
