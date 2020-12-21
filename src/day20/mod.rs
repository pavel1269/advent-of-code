mod directions;
mod map_connector;
mod parser;
mod map_tile;
use map_connector::*;
use parser::*;

pub fn get_part1_result() -> i64 {
    let _ = get_challenge_input();
    let input = get_example_input();
    let mut map = parse_input(input);
    connect_map(&mut map);
    // println!("{:?}", map);
    return -1;
}

pub fn get_challenge_input() -> &'static str {
    include_str!("input.txt")
}

pub fn get_example_input() -> &'static str {
    include_str!("example.txt")
}
