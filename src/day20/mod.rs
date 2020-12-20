mod directions;
mod parser;
use parser::*;

pub fn get_part1_result() -> i64 {
    let _ = get_challenge_input();
    let input = get_example_input();
    let map = parse_input(input);
    println!("{:?}", map);
    return -1;
}

pub fn get_challenge_input() -> &'static str {
    include_str!("input.txt")
}

pub fn get_example_input() -> &'static str {
    include_str!("example.txt")
}
