fn main() {
    let input = get_input();
    parse_input(input);
    
    // let result_part1 = calculate_part1(&some_input);
    // println!("Part1: {}", result_part1);
}

fn parse_input(input: &str) {
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
        parse_input(input);
        // let result = calculate_part1(&some_input);
        // assert_eq!(result, 0);
    }
}
