pub struct Crt;

impl Crt {
    pub fn draw_pixel(current_cycle: i32, sprite_middle: i32) {
        print!("{}", Crt::get_pixel(current_cycle, sprite_middle));

        if Crt::needs_new_line(current_cycle) {
            println!();
        }
    }

    fn get_pixel(current_cycle: i32, sprite_middle: i32) -> char {
        if Crt::sprite_in_range(current_cycle, sprite_middle) {
            '#'
        } else {
            ' '
        }
    }

    fn sprite_in_range(current_cycle: i32, sprite_middle: i32) -> bool {
        (current_cycle % 40 - sprite_middle).abs() < 2
    }

    fn needs_new_line(current_cycle: i32) -> bool {
        (current_cycle + 1) % 40 == 0 && current_cycle != 0
    }
}
