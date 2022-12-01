mod eval;
mod parser;
use eval::*;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = evaluate_eq_sum(input, false);
    return result;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let result = evaluate_eq_sum(input, true);
    return result;
}

fn get_challenge_input() -> &'static str {
    include_str!("input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_part1_result() {
        let result: i64 = get_part1_result();
        assert_eq!(701339185745, result);
    }

    #[test]
    fn input_part2_result() {
        let result: i64 = get_part2_result();
        assert_eq!(4208490449905, result);
    }
}
