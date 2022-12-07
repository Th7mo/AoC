use crate::terminal::Terminal;

pub fn solve() -> u32 {
    let mut terminal = Terminal::new();
    terminal.get_sum_of_dirs_lower_then_threshold(100_000)
}
