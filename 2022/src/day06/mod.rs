use std::collections::HashSet;

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = detect_signal_start(input, 4);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = detect_signal_start(input, 14);
    return result.to_string();
}

fn detect_signal_start(signal: &str, find_lenth: usize) -> usize {
    let mut found: Vec<HashSet<char>> = Vec::with_capacity(4);
    for _ in 0..find_lenth {
        found.push(HashSet::new())
    }

    for (char_index, char) in signal.chars().enumerate() {
        for search_index in 0..find_lenth {
            found[search_index].insert(char);
        }

        let index = char_index % find_lenth;
        if found[index].len() == find_lenth {
            return char_index + 1;
        }
        found[index].clear();
    }

    panic!("Haven't found start of the signal.");
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn get_example_input1() -> &'static str {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
    }
    
    fn get_example_input2() -> &'static str {
        "bvwbjplbgvbhsrlpgdmjqwftvncz"
    }
    
    fn get_example_input3() -> &'static str {
        "nppdvjthqldpwncqszvftbrmjlhg"
    }
    
    fn get_example_input4() -> &'static str {
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
    }
    
    fn get_example_input5() -> &'static str {
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
    }
    
    #[test_case(get_example_input1, 7; "input1")]
    #[test_case(get_example_input2, 5; "input2")]
    #[test_case(get_example_input3, 6; "input3")]
    #[test_case(get_example_input4, 10; "input4")]
    #[test_case(get_example_input5, 11; "input5")]
    fn part1_example(get_input_fn: fn () -> &'static str, expected_result: usize) {
        let input = get_input_fn();
        let result = detect_signal_start(input, 4);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "1651");
    }

    #[test_case(get_example_input1, 19; "input1")]
    #[test_case(get_example_input2, 23; "input2")]
    #[test_case(get_example_input3, 23; "input3")]
    #[test_case(get_example_input4, 29; "input4")]
    #[test_case(get_example_input5, 26; "input5")]
    fn part2_example(get_input_fn: fn () -> &'static str, expected_result: usize) {
        let input = get_input_fn();
        let result = detect_signal_start(input, 14);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn part2_input() {
        let result = get_solution_part2();

        assert_eq!(result, "3837");
    }
}
