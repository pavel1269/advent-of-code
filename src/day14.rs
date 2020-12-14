
#[path = "day14/mod.rs"] mod day14;
use day14::*;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let state = run_program(input);
    let result = sum_memory(&state);

    return result as i64;
}

fn get_challenge_input() -> &'static str {
    include_str!("./day14/day14.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_get_earliest_bus() {
        let result = get_part1_result();

        assert_eq!(12610010960049, result);
    }
}
