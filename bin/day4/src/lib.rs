pub mod a;
mod assignment;
pub mod b;
mod file_parser;
mod pair;

pub fn i32_of(str: &str) -> i32 {
    str.parse()
        .unwrap_or_else(|_| panic!("Can't convert {str} to a number"))
}
