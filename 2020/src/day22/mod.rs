use std::collections::{HashSet, LinkedList};

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = get_end_score(input, false);
    return result;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let result = get_end_score(input, true);
    return result;
}

fn get_end_score(input: &str, recursive: bool) -> i64 {
    let (mut player1_numbers, mut player2_numbers) = parse_input(input);
    let player1_winner = play(&mut player1_numbers, &mut player2_numbers, recursive, 1);
    let result = if player1_winner {
        println!("Player 1 won");
        count_score(&player1_numbers)
    } else {
        println!("Player 2 won");
        count_score(&player2_numbers)
    };

    return result;
}

fn count_score(numbers: &LinkedList<usize>) -> i64 {
    let mut score: i64 = 0;
    let count = numbers.len() as i64;
    for (index, number) in numbers.iter().enumerate() {
        let score_part = *number as i64 * (count - index as i64);
        score += score_part as i64;
    }

    return score;
}

fn play(
    player1_numbers: &mut LinkedList<usize>,
    player2_numbers: &mut LinkedList<usize>,
    recursive: bool,
    rec_index: usize,
) -> bool {
    let mut known_states: HashSet<String> = HashSet::new();
    while player1_numbers.len() > 0 && player2_numbers.len() > 0 {
        if recursive {
            let id1 = player1_numbers
                .iter()
                .fold(String::new(), |a, b| a + "," + b.to_string().as_str());
            let id2 = player2_numbers
                .iter()
                .fold(String::new(), |a, b| a + "," + b.to_string().as_str());
            let id = id1 + "-" + id2.as_str();
            if known_states.contains(&id) {
                return true;
            }

            known_states.insert(id);
        }

        // println!();
        // println!("[{}] --- Round", rec_index);
        // println!("Player 1 deck: {:?}", player1_numbers);
        // println!("Player 2 deck: {:?}", player2_numbers);

        let number_pl1 = player1_numbers.pop_front().unwrap();
        let number_pl2 = player2_numbers.pop_front().unwrap();
        let sub_game =
            recursive && number_pl1 <= player1_numbers.len() && number_pl2 <= player2_numbers.len();

        // println!("Player 1 plays: {}", number_pl1);
        // println!("Player 2 plays: {}", number_pl2);

        let round_winner_p1 = if sub_game {
            // println!("[{}] --- Starting sub-game", rec_index);
            let result = play(
                &mut player1_numbers.iter().copied().take(number_pl1).collect(),
                &mut player2_numbers.iter().copied().take(number_pl2).collect(),
                true,
                rec_index + 1,
            );
            // println!("[{}] --- Sub-game end", rec_index);

            result
        } else {
            number_pl1 > number_pl2
        };

        if round_winner_p1 {
            // println!("Player 1 won round");
            player1_numbers.push_back(number_pl1);
            player1_numbers.push_back(number_pl2);
        } else {
            // println!("Player 2 won round");
            player2_numbers.push_back(number_pl2);
            player2_numbers.push_back(number_pl1);
        }
    }

    return player1_numbers.len() > 0;
}

fn parse_input(input: &str) -> (LinkedList<usize>, LinkedList<usize>) {
    let input = input.splitn(2, "Player 2:").collect::<Vec<&str>>();
    let player1_input = input[0][10..].trim();
    let player1_numbers = parse_player_input(player1_input);
    let player2_input = input[1].trim();
    let player2_numbers = parse_player_input(player2_input);

    return (player1_numbers, player2_numbers);
}

fn parse_player_input(input: &str) -> LinkedList<usize> {
    let mut numbers: LinkedList<usize> = LinkedList::new();
    for number in input.lines() {
        let number = number.parse::<usize>().unwrap();
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
    fn example_play_score_matches() {
        let input = get_example_input();
        let result = get_end_score(input, false);
        assert_eq!(306, result);
    }

    #[test]
    fn input_play_score_matches() {
        let result = get_part1_result();
        assert_eq!(31629, result);
    }

    #[test]
    fn example_recursive_play_score_matches() {
        let input = get_example_input();
        let result = get_end_score(input, true);
        assert_eq!(291, result);
    }

    #[test]
    fn input_recursive_play_score_matches() {
        time_test::time_test!();
        let result = get_part2_result();
        assert_eq!(35196, result);
    }
}
