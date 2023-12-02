use std::fs;

pub fn get_input(day: u32) -> String {
    return fs::read_to_string(format!("inputs/{}", day))
        .unwrap_or_else(|_| panic!("no puzzle input for day {} found", day));
}