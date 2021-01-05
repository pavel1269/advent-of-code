use std::collections::LinkedList;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = play_cups(input, 100);
    return result;
}

fn play_cups(input: &str, rounds: usize) -> i64 {
    let mut cups = parse_input(input);
    play_rounds(&mut cups, rounds);
    let result = format_result(&cups);
    return result;
}

fn format_result(cups: &LinkedList<u32>) -> i64 {
    let (index, _) = cups.iter().enumerate().find(|(_, cup)| **cup == 1).unwrap();
    let mut cups = cups.clone();
    let mut remaining = cups.split_off(index);
    remaining.pop_front();
    remaining.append(&mut cups);
    let result = remaining.iter().fold(0, |a, b| a * 10 + b);
    return result as i64;
}

fn play_rounds(cups: &mut LinkedList<u32>, rounds: usize) {
    let cups_count = cups.len();
    for _round_index in 1..rounds + 1 {
        debug_assert_eq!(cups_count, cups.len());
        println!("{} cups: {:?}", _round_index, cups);
        let cup_in_order = cups.pop_front().unwrap();
        let mut remaining = cups.split_off(3);
        println!(
            "{} pick: {:?} ({:?}) (cup_in_order: {})",
            _round_index, cups, remaining, cup_in_order
        );

        let mut next_place = cup_in_order - 1;
        if next_place == 0 {
            next_place = cups_count as u32;
        }

        while cups.iter().any(|cup| *cup == next_place) {
            // println!("{} is held", next_place);
            next_place -= 1;
            if next_place == 0 {
                next_place = cups_count as u32;
            }
        }
        debug_assert_ne!(next_place, cup_in_order);
        println!("{} next_place: {}", _round_index, next_place);

        let next_place_index = remaining
            .iter()
            .copied()
            .enumerate()
            .find(|(_, cup)| *cup == next_place)
            .unwrap()
            .0
            + 1 % cups_count;
        println!(
            "{} destination: {} (insert at index {})",
            _round_index, next_place, next_place_index
        );
        println!();

        let mut split = remaining.split_off(next_place_index);
        remaining.append(cups);
        remaining.append(&mut split);
        remaining.push_back(cup_in_order);
        *cups = remaining;
    }

    println!("{} cups: {:?}", rounds + 1, cups);
}

fn parse_input(input: &str) -> LinkedList<u32> {
    let mut cups: LinkedList<u32> = LinkedList::new();
    for char in input.chars().into_iter() {
        let number = char.to_digit(10).unwrap();
        cups.push_back(number);
    }
    return cups;
}

fn get_challenge_input() -> &'static str {
    "583976241"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "389125467"
    }

    #[test]
    fn example_format_result() {
        let input = get_example_input();
        let input = parse_input(input);
        let result = format_result(&input);

        assert_eq!(25467389, result);
    }

    #[test]
    fn example_play_1_round() {
        let input = get_example_input();
        let result = play_cups(input, 1);

        assert_eq!(54673289, result);
    }

    #[test]
    fn example_play_10_rounds() {
        let input = get_example_input();
        let result = play_cups(input, 10);

        assert_eq!(92658374, result);
    }

    #[test]
    fn example_play_100_rounds() {
        let input = get_example_input();
        let result = play_cups(input, 100);

        assert_eq!(67384529, result);
    }

    #[test]
    fn input_part1_result() {
        let result = get_part1_result();

        assert_eq!(24987653, result);
    }
}
