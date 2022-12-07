pub enum Command<'a> {
    Cd(&'a str),
    File(&'a str),
}

impl<'a> Command<'a> {
    pub fn from(command: &'a str) -> Self {
        match command {
            c if c.starts_with('$') => Self::Cd(command),
            _ => Self::File(command),
        }
    }
}
