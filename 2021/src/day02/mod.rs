
use core::panic;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = calculate_part1(input);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = calculate_part2(input);
    return result;
}

fn calculate_part1(moves: Vec<&'static str>) -> i64 {
    let coords = calculate_position(moves);
    return coords.0 as i64 * coords.1 as i64;
}

fn calculate_part2(moves: Vec<&'static str>) -> i64 {
    let coords = calculate_position_aim(moves);
    return coords.0 as i64 * coords.1 as i64;
}

fn calculate_position_aim(moves: Vec<&'static str>) -> (i32, i32) {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    
    for move_command in moves {
        let (direction, speed) = parse_command(move_command);

        match direction.as_str() {
            "up" => aim -= speed,
            "down" => aim += speed,
            "forward" => {
                position += speed;
                depth += aim * speed;
            },
            _ => panic!("Unexpected direction '{}'", &direction),
        }
    }

    return (position, depth);
}

fn calculate_position(moves: Vec<&'static str>) -> (i32, i32) {
    let mut depth = 0;
    let mut position = 0;

    for move_command in moves {
        let (direction, speed) = parse_command(move_command);

        match direction.as_str() {
            "up" => depth -= speed,
            "down" => depth += speed,
            "forward" => position += speed,
            _ => panic!("Unexpected direction '{}'", &direction),
        }
    }

    return (position, depth);
}

fn parse_command(move_command: &str) -> (String, i32) {
    use regex::Regex;

    lazy_static::lazy_static! {
        static ref REGEX: Regex = Regex::new("^(forward|up|down) (\\d+)$").unwrap();
    }

    let captures = match REGEX.captures(move_command) {
        Some(capture) => capture,
        None => panic!("Could not parse '{}'", move_command),
    };
    let direction = &captures[1];
    let speed: i32 = captures[2].parse().unwrap();

    return (direction.to_string(), speed);
}

fn get_input() -> Vec<&'static str> {
    let file_contents = include_str!("./input.txt").lines().collect();
    return file_contents;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> Vec<&'static str> {
        return vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
    }

    #[test]
    fn example_part1_correct_result() {
        let result = calculate_part1(get_example_input());

        assert_eq!(150, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(2039912, result);
    }

    #[test]
    fn example_part2_correct_result() {
        let result = calculate_part2(get_example_input());

        assert_eq!(900, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(1942068080, result);
    }
}
