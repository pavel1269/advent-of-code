fn main() {
    let input = get_input();
    let result_part1 = get_calibration_value(input);

    println!("Part 1: {}", result_part1);
}

fn get_calibration_value(input: &str) -> u32 {
    let regex_first = regex::Regex::new(r"^[a-zA-Z]*(\d)").unwrap();
    let regex_last = regex::Regex::new(r"(\d)[a-zA-Z]*$").unwrap();
    let input_lines = input.lines();
    let mut sum = 0;
    input_lines.for_each(|line| {
        let captures = regex_first.captures(line).unwrap();
        let number_1: u32 = captures[1].parse().unwrap();
        let captures = regex_last.captures(line).unwrap();
        let number_2: u32 = captures[1].parse().unwrap();
        let number = number_1 * 10 + number_2;
        sum += number;
    });

    return sum;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        include_str!("./example.txt")
    }
    
    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = get_calibration_value(input);

        assert_eq!(result, 142);
    }    
}
