
pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = detect_loop(input);

    return result;
}

fn detect_loop(input: &str) -> i64 {
    let instructions = parse_program(input);

    let mut accumulator: i64 = 0;
    let mut program_counter: i64 = 0;

    let mut instructions: Vec<(&Instruction, bool)> = instructions.iter().map(|ins| (ins, false)).collect();
    loop {
        let mut instruction = &mut instructions[program_counter as usize];

        if instruction.1 {
            break;
        }
        instruction.1 = true;

        match instruction.0.operation {
            Operation::Nop => {
                program_counter += 1;
            },
            Operation::Acc => {
                accumulator += instruction.0.argument as i64;
                program_counter += 1;
            },
            Operation::Jmp => {
                program_counter += instruction.0.argument as i64;
            }
        }
    }

    return accumulator;
}

enum Operation {
    Nop = 0,

    Acc,
    Jmp,
}

impl std::str::FromStr for Operation {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "nop" => Ok(Operation::Nop),
            "acc" => Ok(Operation::Acc),
            "jmp" => Ok(Operation::Jmp),
            _ => Err(String::from(format!("Could not parse '{}'", string))),
        }
    }
}

struct Instruction {
    operation: Operation,
    argument: i32,
}

fn parse_program(input: &str) -> Vec<Instruction> {
    use regex::Regex;

    let mut instructions: Vec<Instruction> = vec!();
    let code_regex = Regex::new("^(\\w{3}) ([+-]\\d+)$").unwrap();
    for input_line in input.lines() {
        let captures = code_regex.captures(input_line).unwrap();
        let operation = captures[1].parse::<Operation>().unwrap();
        let argument = captures[2].parse::<i32>().unwrap();

        let operation = Instruction {
            operation: operation,
            argument: argument,
        };
        instructions.push(operation);
    }

    return instructions;
}

fn get_challenge_input() -> &'static str {
    include_str!("./inputs/day08.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example() -> &'static str {
        "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
    }

    #[test]
    fn example_detect_loop() {
        let input = get_example();
        let result = detect_loop(input);

        assert_eq!(5, result);
    }

    #[test]
    fn input_detect_loop() {
        let input = get_challenge_input();
        let result = detect_loop(input);

        assert_eq!(1262, result);
    }
}
