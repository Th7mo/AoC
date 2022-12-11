pub mod a;
pub mod b;
mod file_parser;

pub fn i32_of(str: &str) -> i32 {
    str.parse()
        .unwrap_or_else(|_| panic!("Can't convert {str} to a number"))
}
