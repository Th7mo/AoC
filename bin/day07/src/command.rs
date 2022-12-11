pub enum Command {
    CD(String),
    File(String),
}

impl Command {
    pub fn from(command: String) -> Self {
        match command.chars().next().unwrap() {
            '$' => Self::CD(command),
            _ => Self::File(command),
        }
    }
}
