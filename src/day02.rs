
pub fn get_solution_day02_part1() -> i32 {
    let input = get_part1_input();
    let parsed_input = parse_input(input);
    let result = get_number_of_valid_passwords(parsed_input);

    return result;
}

fn get_number_of_valid_passwords(inputs: Vec<Vec<&str>>) -> i32 {
    let mut count: i32 = 0;

    for line in inputs.iter() {
        let min: usize = line[0].parse().expect(&format!("Min '{}' could not be parsed as a number", line[0]));
        let max: usize = line[1].parse().expect(&format!("Max '{}' could not be parsed as a number", line[1]));
        let character = line[2].chars().next().unwrap();
        let char_count = line[4].matches(character).count();

        // println!("min: {}({}), max: {}({}), letter: {}({}), password: {}x ({})", min, line[0], max, line[1], character, line[2], char_count, line[4]);

        if char_count < min || char_count > max {
            continue;
        }

        count += 1;
    }

    return count;
}

fn parse_input(input: Vec<&str>) -> Vec<Vec<&str>> {
    input.iter().map(|line| line.splitn(5, |s: char| s.is_whitespace() || s == '-' || s== ':').collect()).collect()
}

fn get_part1_input() -> Vec<&'static str> {
    include_str!("./inputs/day02part1.txt").lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn get_example_input() -> Vec<&'static str> {
        let input = vec![
            "1-3 a: abcde",
            "1-3 b: cdefg",
            "2-9 c: ccccccccc",
        ];

        return input;
    }

    #[test]
    fn example_part1() {
        let input = get_example_input();
        let parsed_input = parse_input(input);
        let result = get_number_of_valid_passwords(parsed_input);

        assert_eq!(2, result);
    }

    #[test]
    fn day01_part1() {
        let expected_result = 548;
        let result = get_solution_day02_part1();

        assert_eq!(expected_result, result);
    }
}
