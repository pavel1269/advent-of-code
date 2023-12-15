use std::{
    collections::LinkedList,
    ops::Rem,
};

fn main() {
    let input = get_input();
    let strings = parse_input(input);
    let result_part1 = hash_sequence(&strings);
    println!("Part1: {}", result_part1);
    let result_part2 = part2(input);
    println!("Part2: {}", result_part2);
}

fn hash_sequence(strings: &Vec<String>) -> u32 {
    strings.iter().map(|str| hash(str)).sum()
}

fn part2(input: &str) -> u32 {
    let instructions = parse_input_instructions(input);
    let mut boxes: Vec<LinkedList<Box>> = vec![LinkedList::new(); 256];

    for instruction in instructions {
        match instruction.op {
            Op::Remove => {
                let lens = boxes.get_mut(instruction.hash as usize).unwrap();
                let to_remove = lens
                    .iter()
                    .enumerate()
                    .filter(|(_, b)| b.label == instruction.label)
                    .map(|(index, _)| index)
                    .next();
                if let Some(to_remove) = to_remove {
                    remove_from_linked_list(lens, to_remove);
                }
            }
            Op::Set => {
                let lens = boxes.get_mut(instruction.hash as usize).unwrap();
                let existing = lens
                    .iter_mut()
                    .filter(|b| b.label == instruction.label)
                    .next();
                if let Some(existing) = existing {
                    existing.value = instruction.value.unwrap();
                } else {
                    lens.push_back(Box {
                        label: instruction.label,
                        value: instruction.value.unwrap(),
                    });
                }
            }
        }
    }

    let result = boxes
        .iter()
        .enumerate()
        .map(|(index_box, boxes)| {
            boxes
                .iter()
                .enumerate()
                .map(|(index_in_box, b)| {
                    (index_box + 1) as u32 * (index_in_box + 1) as u32 * b.value
                })
                .sum::<u32>()
        })
        .sum();
    return result;
}

fn remove_from_linked_list<T>(list: &mut LinkedList<T>, index: usize) {
    let mut list2 = list.split_off(index);
    list2.pop_front();
    list.append(&mut list2);
}

#[derive(Debug, Clone)]
struct Box {
    label: String,
    value: u32,
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

struct Instruction {
    label: String,
    hash: u32,
    op: Op,
    value: Option<u32>,
}

impl Instruction {
    fn from(str: &String) -> Self {
        if let Some(_) = str.find('-') {
            let label = str.get(..str.len() - 1).unwrap().to_string();
            let hash = hash(&label);
            return Self {
                label,
                hash,
                op: Op::Remove,
                value: None,
            };
        } else if let Some(index) = str.find('=') {
            let (a, b) = str.split_at(index);
            let label = a.to_string();
            let hash = hash(&label);
            let value = b.get(1..).unwrap().parse().unwrap();
            return Self {
                label,
                hash,
                op: Op::Set,
                value: Some(value),
            };
        } else {
            panic!();
        }
    }
}

enum Op {
    Set,
    Remove,
}

fn parse_input_instructions(input: &str) -> Vec<Instruction> {
    parse_input(input)
        .iter()
        .map(|str| Instruction::from(str))
        .collect()
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
    use test_case::test_case;

    #[test_case("HASH", 52)]
    #[test_case("rn", 0)]
    #[test_case("cm", 0)]
    #[test_case("qp", 1)]
    fn hash_test(str: &str, expect: u32) {
        let result = hash(&str.to_string());
        assert_eq!(result, expect);
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

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = part2(input);
        assert_eq!(result, 145);
    }
}
