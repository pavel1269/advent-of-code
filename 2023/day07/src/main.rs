use std::{collections::HashMap, cmp::Ordering};

fn main() {
    let input = get_input();
    let plays = parse_input(input);
    
    let result_part1 = sum_winnings(&plays);
    println!("Part1: {}", result_part1);
}

fn sum_winnings(plays: &Vec<Play>) -> u64 {
    let mut plays = plays.clone();
    plays.sort_by(|a, b| a.hand.cmp(&b.hand));
    let mut winnings = 0;
    for (index, play) in plays.iter().enumerate() {
        winnings += (index as u64 + 1) * play.bid;
    }
    return winnings;
}

#[derive(Debug, Clone, Copy)]
struct Play {
    hand: Hand,
    bid: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand {
    cards: [u32; 5],
    score: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.score > other.score {
            return Some(Ordering::Greater);
        }
        else if self.score < other.score {
            return Some(Ordering::Less);
        }

        for index in 0..5 {
            if self.cards[index] > other.cards[index] {
                return Some(Ordering::Greater);
            }
            else if self.cards[index] < other.cards[index] {
                return Some(Ordering::Less);
            }
        }
        return Some(Ordering::Equal);
    }
}

impl Hand {
    fn from(cards: [u32; 5]) -> Self {
        Hand {
            cards,
            score: Hand::score(&cards),
        }
    }

    fn score(cards: &[u32; 5]) -> u32 {
        let mut occurences = HashMap::new();
        for card in cards {
            occurences.entry(*card).and_modify(|val| *val += 1).or_insert(1 as u8);
        }
        let mut duplicates = HashMap::new();
        for occurence in occurences.values() {
            duplicates.entry(*occurence).and_modify(|val| *val += 1).or_insert(1 as u8);
        }
        
        if duplicates.contains_key(&5) {
            return 10;
        }
        else if duplicates.contains_key(&4) {
            return 9;
        }
        else if duplicates.contains_key(&3) && duplicates.contains_key(&2) {
            return 8;
        }
        else if duplicates.contains_key(&3) {
            return 7;
        }
        else if duplicates.contains_key(&2) && duplicates[&2] == 2 {
            return 6;
        }
        else if duplicates.contains_key(&2) {
            return 5;
        }
        return 4;
    }
}

fn parse_input(input: &str) -> Vec<Play> {
    let mut plays = Vec::new();
    for line in input.lines() {
        let line_parts: Vec<&str> = line.split(' ').collect();
        let hand = parse_hand(line_parts[0]);
        let bid: u64 = line_parts[1].parse().unwrap();
        let play = Play {
            hand,
            bid,
        };
        plays.push(play);
    }
    return plays;
}

fn parse_hand(input: &str) -> Hand {
    let mut cards = [0; 5];
    for (index, char) in input.chars().enumerate() {
        match char {
            'A' => cards[index] = 14,
            'K' => cards[index] = 13,
            'Q' => cards[index] = 12,
            'J' => cards[index] = 11,
            'T' => cards[index] = 10,
            '2'..='9' => cards[index] = char as u32 - '0' as u32,
            _ => panic!(),
        }
    }
    return Hand::from(cards);
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

    fn get_example2_input() -> &'static str {
        include_str!("./example2.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let plays = parse_input(input);
        let result = sum_winnings(&plays);
        assert_eq!(result, 6440);
    }

    #[test]
    fn part1_example2() {
        let input = get_example2_input();
        let plays = parse_input(input);
        let result = sum_winnings(&plays);
        assert_eq!(result, 6592);
    }
}
