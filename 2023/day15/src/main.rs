use std::ops::Rem;

fn main() {
    let input = get_input();
    let strings = parse_input(input);
    let result_part1 = hash_sequence(&strings);
    println!("Part1: {}", result_part1);
}

fn hash_sequence(strings: &Vec<String>) -> u32 {
    strings.iter().map(|str| hash(str)).sum()
}

fn hash(str: &String) -> u32 {
    let mut hash = 0;
    for char in str.chars() {
        hash += char as u32;
        hash *= 17;
        hash = hash.rem(256);
    }
    return hash;
}

fn parse_input(input: &str) -> Vec<String> {
    input.trim().split(',').map(|str| str.to_string()).collect()
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        let result = hash(&"HASH".to_string());
        assert_eq!(result, 52);
    }

    fn get_example_input() -> &'static str {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let strings = parse_input(input);
        let result = hash_sequence(&strings);
        assert_eq!(result, 1320);
    }
}
