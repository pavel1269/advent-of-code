use std::collections::LinkedList;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = play_cups(input, 100);
    return result;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let result = play_cups_and_find_stars(input, 10000000);
    return result;
}

fn play_cups_and_find_stars(input: &str, rounds: usize) -> i64 {
    let cups = parse_input_milion(input);
    debug_assert_eq!(1000000, cups.len());
    let mut assoc_array = convert_to_assoc_array(&cups);
    play_rounds(&mut assoc_array, *cups.front().unwrap(), rounds);

    let star_cup_1 = assoc_array[1];
    let star_cup_2 = assoc_array[star_cup_1] as i64;
    let result = star_cup_1 as i64 * star_cup_2;
    // println!("{} * {} = {}", star_cup_1, star_cup_2, result);

    return result;
}

fn play_cups(input: &str, rounds: usize) -> i64 {
    let cups = parse_input(input);
    let mut assoc_array = convert_to_assoc_array(&cups);
    play_rounds(&mut assoc_array, *cups.front().unwrap(), rounds);
    let result = format_result(&assoc_array);
    return result;
}

fn format_result(assoc_array: &Vec<usize>) -> i64 {
    let mut cup = assoc_array[1];
    let mut nubmers = Vec::with_capacity(assoc_array.len());
    while cup != 1 {
        nubmers.push(cup);
        cup = assoc_array[cup];
    }

    let result = nubmers.iter().fold(0, |a, b| a * 10 + b);
    return result as i64;
}

fn play_rounds(assoc_array: &mut Vec<usize>, first: u32, rounds: usize) {
    let max = assoc_array.len() - 1;
    let mut first = first as usize;
    for _round_index in 1..rounds + 1 {
        // println!("{}/{} taking from: {} - {:?}", _round_index, rounds, first, assoc_array);
        let p1 = assoc_array[first];
        let p2 = assoc_array[p1];
        let p3 = assoc_array[p2];
        assoc_array[first] = assoc_array[p3];

        let mut destination = first - 1;
        if destination == 0 {
            destination = max;
        }
        while [p1, p2, p3].contains(&destination) {
            destination = destination - 1;
            if destination == 0 {
                destination = max;
            }
        }
        // println!("{} picked up: {} {} {} -> dest: {}", _round_index, p1, p2, p3, destination);
        // println!();

        let former = assoc_array[destination];
        assoc_array[destination] = p1;
        assoc_array[p3] = former;

        first = assoc_array[first];
    }
    // println!("{:?}", assoc_array);
}

fn convert_to_assoc_array(cups: &LinkedList<u32>) -> Vec<usize> {
    let mut vec = vec![0; cups.len() + 1];
    let mut previous_cup = *cups.back().unwrap() as usize;
    for cup in cups.iter().copied() {
        let cup = cup as usize;
        vec[previous_cup] = cup;
        previous_cup = cup;
    }

    return vec;
}

fn parse_input_milion(input: &str) -> LinkedList<u32> {
    let mut cups: LinkedList<u32> = LinkedList::new();
    for char in input.chars().into_iter() {
        let number = char.to_digit(10).unwrap();
        cups.push_back(number);
    }

    for num in cups.len() + 1..1000001 {
        cups.push_back(num as u32);
    }
    return cups;
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
    fn example_parse_assoc_array() {
        let input = get_example_input();
        let input = parse_input(input);
        let assoc_array = convert_to_assoc_array(&input);

        assert_eq!(vec![0, 2, 5, 8, 6, 4, 7, 3, 9, 1], assoc_array);
    }

    #[test]
    fn example_format_result() {
        let input = get_example_input();
        let input = parse_input(input);
        let assoc_array = convert_to_assoc_array(&input);
        let result = format_result(&assoc_array);

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

    #[test]
    fn example_parse_input_milion_works() {
        let cups = parse_input_milion("123456789");

        assert_eq!(1000000, cups.len());
        assert_eq!(1, *cups.front().unwrap());
        assert_eq!(10, *cups.iter().nth(9).unwrap());
        assert_eq!(1000000, *cups.iter().last().unwrap());
    }

    #[test]
    fn example_part2_result() {
        time_test::time_test!();

        let input = get_example_input();
        let result = play_cups_and_find_stars(input, 10000000);

        assert_eq!(149245887792, result);
    }

    #[test]
    fn input_part2_result() {
        time_test::time_test!();

        let result = get_part2_result();

        assert_eq!(442938711161, result);
    }
}
