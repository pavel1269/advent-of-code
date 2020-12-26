use std::collections::LinkedList;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = get_end_score(input);
    return result;
}

fn get_end_score(input: &str) -> i64 {
    let (mut player1_numbers, mut player2_numbers) = parse_input(input);
    play(&mut player1_numbers, &mut player2_numbers);

    let result = if player1_numbers.len() == 0 {
        println!("Player 2 won");
        count_score(&player2_numbers)
    } else {
        println!("Player 1 won");
        count_score(&player1_numbers)
    };

    return result;
}

fn count_score(numbers: &LinkedList<u32>) -> i64 {
    let mut score: i64 = 0;
    let count = numbers.len() as i64;
    for (index, number) in numbers.iter().enumerate() {
        let score_part = *number as i64 * (count - index as i64);
        score += score_part as i64;
    }

    return score;
}

fn play(player1_numbers: &mut LinkedList<u32>, player2_numbers: &mut LinkedList<u32>) {
    while player1_numbers.len() > 0 && player2_numbers.len() > 0 {
        let number_pl1 = player1_numbers.pop_front().unwrap();
        let number_pl2 = player2_numbers.pop_front().unwrap();

        if number_pl1 > number_pl2 {
            player1_numbers.push_back(number_pl1);
            player1_numbers.push_back(number_pl2);
        } else {
            player2_numbers.push_back(number_pl2);
            player2_numbers.push_back(number_pl1);
        }
    }
}

fn parse_input(input: &str) -> (LinkedList<u32>, LinkedList<u32>) {
    let input = input.splitn(2, "Player 2:").collect::<Vec<&str>>();
    let player1_input = input[0][10..].trim();
    let player1_numbers = parse_player_input(player1_input);
    let player2_input = input[1].trim();
    let player2_numbers = parse_player_input(player2_input);

    // println!("{:?}", player1_numbers);
    // println!("{:?}", player2_numbers);

    return (player1_numbers, player2_numbers);
}

fn parse_player_input(input: &str) -> LinkedList<u32> {
    let mut numbers: LinkedList<u32> = LinkedList::new();
    for number in input.lines() {
        let number = number.parse::<u32>().unwrap();
        numbers.push_back(number);
    }
    return numbers;
}

fn get_challenge_input() -> &'static str {
    include_str!("input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        include_str!("example.txt")
    }

    #[test]
    fn example_score_matches() {
        let input = get_example_input();
        let result = get_end_score(input);
        assert_eq!(306, result);
    }

    #[test]
    fn input_score_matches() {
        let result = get_part1_result();
        assert_eq!(31629, result);
    }
}
