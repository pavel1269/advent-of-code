
pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = play_memory_game(input, 2020);
    return result;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let result = play_memory_game(input, 30000000);
    return result;
}

fn play_memory_game(input: &str, iterations: usize) -> i64 {
    use std::collections::HashMap;

    let numbers_vec = parse_input(input);
    let mut numbers: HashMap<usize, usize> = HashMap::new();
    for (index, number) in numbers_vec[..numbers_vec.len() - 1].iter().enumerate() {
        numbers.insert(*number, index);
    }

    let mut last_number: usize = *numbers_vec.last().unwrap();
    for index in numbers.len()..iterations - 1 {
        // println!("{} Last number: {},  data: {:?}", index, last_number, numbers.get(&last_number));

        match numbers.get(&last_number) {
            None => {
                numbers.insert(last_number, index);
                last_number = 0;
            },
            Some(last_index) => {
                let result = index - last_index;
                numbers.insert(last_number, index);
                last_number = result;
            },
        }
    }

    // println!("{:?}", numbers);
    return last_number as i64;
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split(",").map(|entry| entry.parse::<usize>().unwrap()).collect()
}

fn get_challenge_input() -> &'static str {
    "7,14,0,17,11,1,2"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_play_memory_game() {
        let input = "0,3,6";
        let result = play_memory_game(input, 2020);

        assert_eq!(436, result);
    }

    #[test]
    fn example2_play_memory_game() {
        let input = "1,3,2";
        let result = play_memory_game(input, 2020);

        assert_eq!(1, result);
    }

    #[test]
    fn input_get_part1_result() {
        let result = get_part1_result();

        assert_eq!(206, result);
    }

    #[test]
    fn input_get_part2_result() {
        time_test::time_test!();
        let result = get_part2_result();

        assert_eq!(955, result);
    }
}
