use crate::day07::device::Terminal;

pub fn solve() -> u32 {
    let mut terminal = Terminal::new();
    terminal.get_dir_size_needed_to_update(70_000_000, 30_000_000)
}
