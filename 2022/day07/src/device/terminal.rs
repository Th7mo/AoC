use crate::device::Command;
use crate::device::Dir;
use crate::device::DirStack;
use crate::device::FileManager;

pub struct Terminal {
    threshold: Option<u32>,
    dir_stack: DirStack,
    file_manager: FileManager,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            threshold: None,
            dir_stack: DirStack::new(),
            file_manager: FileManager::new(),
        }
    }

    pub fn set_threshold(&mut self, threshold: Option<u32>) {
        self.threshold = threshold;
    }

    pub fn get_size_of_dirs(&mut self) -> u32 {
        self.calculate_size_of_dirs();
        self.file_manager.sum_of_all_dirs()
    }

    fn calculate_size_of_dirs(&mut self) {
        let commands = include_str!("../../res/input.txt");
        self.execute_commands(commands);
    }

    fn execute_commands(&mut self, commands: &str) {
        for terminal_line in commands.lines() {
            if Terminal::is_useless_command(terminal_line) {
                continue;
            }
            let line = Command::from(terminal_line.to_string());
            self.interpreter_line(line);
        }
        self.drain_stack_remainder();
    }

    fn is_useless_command(command: &str) -> bool {
        command.starts_with("$ ls") || command.starts_with("dir")
    }

    fn interpreter_line(&mut self, line: Command) {
        match line {
            Command::CD(line) => self.change_directory(&line),
            Command::File(line) => self.dir_stack.add_size_to_current_dir(&line),
        }
    }

    fn change_directory(&mut self, line: &str) {
        let dir_name = line
            .split_once("$ cd ")
            .expect("incorrect instruction set")
            .1;
        if Terminal::is_move_up_command(dir_name) {
            let popped_dir = self.dir_stack.move_up_dir();
            self.add_to_file_manager(popped_dir);
        } else {
            self.dir_stack.add_new_dir();
        }
    }

    fn is_move_up_command(dir_name: &str) -> bool {
        dir_name == ".."
    }

    fn add_to_file_manager(&mut self, dir: Dir) {
        if self.threshold.is_some() && !dir.smaller_than(self.threshold.unwrap()) {
            return;
        }

        self.file_manager.add_dir(dir);
    }

    pub fn drain_stack_remainder(&mut self) {
        for _ in 0..self.dir_stack.size() {
            let dropped_dir = self.dir_stack.move_up_dir();
            self.add_to_file_manager(dropped_dir);
        }
    }

    pub fn get_dir_size_needed_to_update(&mut self, disk_space: u32, required: u32) -> u32 {
        self.calculate_size_of_dirs();
        self.file_manager
            .get_dir_size_for_removal(disk_space, required)
    }
}
