
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
    let mut numbers: HashMap<i64, (usize, Option<usize>)> = HashMap::new();
    for (index, number) in numbers_vec.iter().enumerate() {
        numbers.insert(*number, (index, None));
    }

    let mut last_number: i64 = *numbers_vec.last().unwrap();
    for index in numbers.len()..iterations {
        let entry = &numbers[&last_number];
        // println!("Last number: {}, data: {:?}", last_number, entry);
        
        last_number = match entry.1 {
            Some(previous_number) => (entry.0 - previous_number) as i64,
            None => 0,
        };

        match numbers.get(&last_number) {
            Some(number_entry) => {
                let last_index = number_entry.0;
                numbers.insert(last_number, (index, Some(last_index)));
            },
            None => {
                numbers.insert(last_number, (index, None));
            },
        }

        // println!("{} New last number: {},  data: {:?}", index, last_number, numbers[&last_number]);
    }

    // println!("{:?}", numbers);
    return last_number;
}

fn parse_input(input: &str) -> Vec<i64> {
    input.split(",").map(|entry| entry.parse::<i64>().unwrap()).collect()
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
