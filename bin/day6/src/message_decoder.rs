use std::collections::HashSet;

pub fn print_first_n_unique_character(message: &str, amount_of_unique_chars: usize) {
    let windows = message.as_bytes().windows(amount_of_unique_chars);

    for (index, window) in windows.enumerate() {
        let set = get_unique_chars(window);
        if set.len() == amount_of_unique_chars {
            println!("{}", index + amount_of_unique_chars);
            break;
        }
    }
}

fn get_unique_chars(chars: &[u8]) -> HashSet<u8> {
    HashSet::from_iter(chars.iter().cloned())
}
