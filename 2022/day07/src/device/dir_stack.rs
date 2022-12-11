use crate::device::Dir;

pub struct DirStack {
    stack: Vec<Dir>,
}

impl DirStack {
    pub fn new() -> Self {
        DirStack { stack: Vec::new() }
    }

    pub fn move_up_dir(&mut self) -> Dir {
        let latest_dir = self.stack.pop().expect("can't move out of / dir");
        let latest_dir_size = latest_dir.size();
        if let Some(dir) = self.stack.last_mut() {
            dir.add(latest_dir_size)
        }
        latest_dir
    }

    pub fn add_new_dir(&mut self) {
        self.stack.push(Dir::new());
    }

    pub fn add_size_to_current_dir(&mut self, line: &str) {
        let file_size = line.split_once(' ').expect("incorrect instruction set").0;
        let file_size_as_u32: u32 = file_size.parse().unwrap();
        self.stack
            .last_mut()
            .expect("stack is empty")
            .add(file_size_as_u32);
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }
}
