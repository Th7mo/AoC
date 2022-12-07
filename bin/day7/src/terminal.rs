use crate::dir::Dir;
use crate::dir_stack::DirStack;
use crate::file_manager::FileManager;
use crate::line::Line;

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
        self.file_manager.size_of_all_dirs()
    }

    fn calculate_size_of_dirs(&mut self) {
        let commands = include_str!("input.txt");
        self.execute_commands(commands);
    }

    fn execute_commands(&mut self, commands: &str) {
        for terminal_line in commands.lines() {
            let line = Line::from(terminal_line);
            self.interpreter_line(line);
        }
        self.drain_stack_remainder();
    }

    pub fn drain_stack_remainder(&mut self) {
        for _ in 0..self.dir_stack.size() {
            let dropped_dir = self.dir_stack.move_up_dir();
            self.add_to_file_manager(dropped_dir);
        }
    }

    fn interpreter_line(&mut self, line: Line) {
        match line {
            Line::Cd(line) => self.change_directory(line),
            Line::ListFile(line) => self.dir_stack.add_size_to_current_dir(line),
            _ => (),
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
        match self.threshold {
            Some(threshold) => {
                if dir.smaller_than(threshold) {
                    self.file_manager.add_dir(dir);
                }
            }
            None => self.file_manager.add_dir(dir),
        }
    }

    pub fn get_dir_size_needed_to_update(&mut self, disk_space: u32, required: u32) -> u32 {
        self.calculate_size_of_dirs();
        self.file_manager
            .get_dir_size_needed_to_update(disk_space, required)
    }
}
