use std::fs;

pub fn read_input(day: u32) -> String {
    let input = fs::read_to_string(format!("src/input/{}.txt", day))
        .expect("Failed to read file");
    input
}