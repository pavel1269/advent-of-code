fn main() {
    let input = get_input();
    let result_part1 = get_calibration_value(input);
    println!("Part 1: {}", result_part1);
    let result_part2 = get_calibration_value_part2(input);
    println!("Part 2: {}", result_part2);
}

fn get_calibration_value_part2(input: &str) -> u32 {
    let regex_first = regex::Regex::new(r"^[a-zA-Z]*(\d)").unwrap();
    let regex_last = regex::Regex::new(r"(\d)[a-zA-Z]*$").unwrap();
    let input_lines = input.lines();
    let mut sum = 0;
    input_lines.for_each(|line| {
        let line = line.replace("one", "on1ne")
            .replace("two", "tw2wo")
            .replace("three", "thre3hree")
            .replace("four", "fou4our")
            .replace("five", "fiv5ive")
            .replace("six", "si6ix")
            .replace("seven", "seve7even")
            .replace("eight", "eigh8ight")
            .replace("nine", "nin9ine");
        let captures = regex_first.captures(&line).unwrap();
        let number_1: u32 = captures[1].parse().unwrap();
        let captures = regex_last.captures(&line).unwrap();
        let number_2: u32 = captures[1].parse().unwrap();
        let number = number_1 * 10 + number_2;
        sum += number;
    });

    return sum;
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

    fn get_example_input_part1() -> &'static str {
        include_str!("./example_part1.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example_input_part1();
        let result = get_calibration_value(input);

        assert_eq!(result, 142);
    }

    fn get_example_input_part2() -> &'static str {
        include_str!("./example_part2.txt")
    }

    #[test]
    fn part2_example() {
        let input = get_example_input_part2();
        let result = get_calibration_value_part2(input);

        assert_eq!(result, 281);
    }
}
