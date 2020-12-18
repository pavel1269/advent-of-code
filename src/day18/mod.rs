mod parser;
use parser::*;

pub fn get_part1_result() -> i64 {
    let input = get_example_eq();
    parse_eq(input);
    return -1;
}

fn get_example_eq() -> &'static str {
    "1 + 2 * 3 + 4 * 5 + 6"
}
