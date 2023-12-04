fn main() {
    let input = get_input();
    let lottery_cards = parse_input(input);
    let result_part1 = calculate_winning_points(&lottery_cards);
    println!("Part1: {}", result_part1);
}

fn calculate_winning_points(lottery_cards: &Vec<LotteryCard>) -> u32 {
    let mut points = 0;
    for card in lottery_cards {
        let mut winners = 0;
        for number in card.numbers.iter() {
            if card.numbers_winning.contains(&number) {
                winners += 1;
            }
        }

        if winners > 0 {
            points += u32::pow(2, winners - 1);
        }
    }
    return points;
}

#[derive(Debug)]
struct LotteryCard {
    numbers: Vec<u32>,
    numbers_winning: Vec<u32>,
}

fn parse_input(input: &str) -> Vec<LotteryCard> {
    let regex = regex::Regex::new(r"^Card +(\d+): (.+) \| (.+)$").unwrap();
    let mut cards = Vec::new();
    for line in input.lines() {
        let captures = regex.captures(line).unwrap();

        let numbers_winning_string = &captures[2];
        let numbers_string = &captures[3];
        
        let numbers_winning = parse_numbers_string(numbers_winning_string);
        let numbers = parse_numbers_string(numbers_string);

        cards.push(LotteryCard { numbers, numbers_winning });
    }
    return cards;
}

fn parse_numbers_string(numbers_string: &str) -> Vec<u32> {
    let numbers = numbers_string.split(' ').filter(|str| str.len() > 0).map(|str| str.trim().parse().unwrap()).collect();
    return numbers;
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
        let lottery_cards = parse_input(input);
        let result = calculate_winning_points(&lottery_cards);

        assert_eq!(result, 13);
    }
}
