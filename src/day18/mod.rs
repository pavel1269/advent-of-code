mod eval;
mod parser;
use eval::*;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let input = get_example_eq();
    let result = evaluate_eq(input);
    return result;
}

fn get_example_eq() -> &'static str {
    "1 + 2 * 3 + 4 * 5 + 6"
}

fn get_challenge_input() -> &'static str {
    include_str!("input.txt")
}
