use crate::dir::Dir;
use crate::line::Line;

pub struct Terminal {
    stack: Vec<Dir>,
    threshold: u32,
    dirs_lower_than_threshold: Vec<Dir>,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            stack: Vec::new(),
            threshold: 0,
            dirs_lower_than_threshold: Vec::new(),
        }
    }

    pub fn get_sum_of_dirs_lower_then_threshold(&mut self, threshold: u32) -> u32 {
        self.threshold = threshold;
        let commands = include_str!("input.txt");
        self.execute_commands(commands);
        self.size_of_all_dirs()
    }

    fn execute_commands(&mut self, commands: &str) {
        for terminal_line in commands.lines() {
            let line = Line::from(terminal_line);
            self.interpreter_line(line);
        }
        self.drain_stack_remainder();
    }

    fn interpreter_line(&mut self, line: Line) {
        match line {
            Line::Cd(line) => self.change_directory(line),
            Line::Ls(_) => (),
            Line::ListFile(line) => self.add_to_current_dir(line),
            Line::ListDir(_) => (),
        }
    }

    fn change_directory(&mut self, line: &str) {
        let dir_name = line
            .split_once("$ cd ")
            .expect("incorrect instruction set")
            .1;
        if Terminal::is_move_up_command(dir_name) {
            self.move_up_dir();
        } else {
            self.add_new_dir();
        }
    }

    fn is_move_up_command(dir_name: &str) -> bool {
        dir_name == ".."
    }

    fn move_up_dir(&mut self) {
        let latest_dir = self.stack.pop().expect("can't move out of / dir");
        let latest_dir_size = latest_dir.size();
        if latest_dir.smaller_than(self.threshold) {
            self.dirs_lower_than_threshold.push(latest_dir);
        }

        self.stack.last_mut().unwrap().add(latest_dir_size);
    }

    fn add_new_dir(&mut self) {
        self.stack.push(Dir::new());
    }

    fn add_to_current_dir(&mut self, line: &str) {
        let file_size = line.split_once(' ').expect("incorrect instruction set").0;
        let file_size_as_u32: u32 = file_size.parse().unwrap();
        self.stack
            .last_mut()
            .expect("stack is empty")
            .add(file_size_as_u32);
    }

    fn drain_stack_remainder(&mut self) {
        for _ in 0..self.stack.len() - 1 {
            self.move_up_dir();
        }
    }

    fn size_of_all_dirs(&self) -> u32 {
        self.dirs_lower_than_threshold
            .iter()
            .map(|dir| dir.size())
            .sum()
    }

    pub fn get_dir_size_needed_to_update(&mut self, disk_space: u32, required: u32) -> u32 {
        self.get_sum_of_dirs_lower_then_threshold(disk_space);
        let used_size = self.stack.first().unwrap().size();
        let target_size = required - (disk_space - used_size);
        self.get_lowest_dir_candidate(target_size)
    }

    fn get_lowest_dir_candidate(&mut self, target_size: u32) -> u32 {
        let mut closest_size_to_target = u32::MAX;
        for dir in &self.dirs_lower_than_threshold {
            if dir.size() > target_size && dir.smaller_than(closest_size_to_target) {
                closest_size_to_target = dir.size();
            }
        }
        closest_size_to_target
    }
}
