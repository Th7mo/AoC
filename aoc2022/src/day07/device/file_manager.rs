use crate::day07::device::Dir;

pub struct FileManager {
    directories: Vec<Dir>,
}

impl FileManager {
    pub fn new() -> Self {
        FileManager {
            directories: Vec::new(),
        }
    }

    pub fn add_dir(&mut self, dir: Dir) {
        self.directories.push(dir);
    }

    pub fn sum_of_all_dirs(&self) -> u32 {
        self.directories.iter().map(|dir| dir.size()).sum()
    }

    pub fn get_dir_size_for_removal(&self, disk_space: u32, required: u32) -> u32 {
        let target_size = required - (disk_space - self.used_size());
        self.get_smallest_dir_for_removal(target_size)
    }

    fn used_size(&self) -> u32 {
        self.directories.last().unwrap().size()
    }

    fn get_smallest_dir_for_removal(&self, target_size: u32) -> u32 {
        let mut closest_size_to_target = u32::MAX;
        for dir in &self.directories {
            if dir.size() > target_size && dir.smaller_than(closest_size_to_target) {
                closest_size_to_target = dir.size();
            }
        }
        closest_size_to_target
    }
}
