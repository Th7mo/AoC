pub enum Command {
    CD(String),
    File(String),
}

impl Command {
    pub fn from(command: &str) -> Self {
        let command = command.to_string();
        match command {
            c if c.starts_with('$') => Self::CD(command),
            _ => Self::File(command),
        }
    }
}
