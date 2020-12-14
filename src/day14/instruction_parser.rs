
use super::{Instruction, MemoryInstruction, MaskInstruction};
use regex::Regex;

struct ParseRegexes {
    mask_regex: Regex,
    memory_regex: Regex,
}

impl Default for ParseRegexes {
    fn default() -> Self {
        Self {
            mask_regex: Regex::new("^mask = ([X01]+)$").unwrap(),
            memory_regex: Regex::new("^mem\\[(\\d+)\\] = (\\d+)$").unwrap(),
        }
    }
}

pub fn parse_input_program(input: &str) -> Vec<Box<dyn Instruction>> {
    let parse_regexes = ParseRegexes::default();
    let mut program: Vec<Box<dyn Instruction>> = Vec::new();
    for line in input.lines() {
        if parse_regexes.mask_regex.is_match(line) {
            let instruction = parse_mask_instruction(&parse_regexes, line);
            let instruction_box = Box::from(instruction);
            program.push(instruction_box);
        } else if parse_regexes.memory_regex.is_match(line) {
            let instruction = parse_memory_instruction(&parse_regexes, line);
            let instruction_box = Box::from(instruction);
            program.push(instruction_box);
        } else {
            panic!("Could not parse instruction");
        }
    }

    return program;
}

fn parse_memory_instruction(parse_regexes: &ParseRegexes, line: &str) -> impl Instruction {
    let captures = parse_regexes.memory_regex.captures(line).unwrap();
    MemoryInstruction {
        address: captures[1].parse().unwrap(),
        value: captures[2].parse().unwrap(),
    }
}

fn parse_mask_instruction(parse_regexes: &ParseRegexes, line: &str) -> impl Instruction {
    let captures = parse_regexes.mask_regex.captures(line).unwrap();
    let mut instruction = MaskInstruction::default();
    let len = captures[1].len();
    for (index, char) in captures[1].chars().enumerate() {
        match char {
            '0' => {
                // a = 11111
                // b = 00010
                // f = 11101
                //
                // 0 0 = 0
                // 0 1 = 0
                // 1 0 = 1
                // 1 1 = 0
                // f = a & !b
                let mask_change = (1 as u64) << (len - index - 1);
                instruction.mask_and &= !mask_change;
            },
            '1' => {
                // a = 00000
                // b = 00010
                // f = 00010
                let mask_change = (1 as u64) << (len - index - 1);
                instruction.mask_or |= mask_change;
            }
            'X' => continue,
            _ => panic!("Unexpected token"),
        };
    }

    return instruction;
}
