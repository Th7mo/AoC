pub struct Crt;

impl Crt {
    pub fn draw_pixel(current_cycle: i32, sprite_middle: i32) {
        if Crt::sprite_in_range(current_cycle, sprite_middle) {
            print!("#");
        } else {
            print!(".");
        }

        if Crt::needs_new_line(current_cycle) {
            println!();
        }
    }

    fn sprite_in_range(current_cycle: i32, sprite_middle: i32) -> bool {
        (current_cycle % 40 - sprite_middle).abs() < 2
    }

    fn needs_new_line(current_cycle: i32) -> bool {
        (current_cycle + 1) % 40 == 0 && current_cycle != 0
    }
}
