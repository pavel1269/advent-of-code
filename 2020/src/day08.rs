
pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = detect_loop_input(input);

    return result;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let result = fix_and_detect_loop(input);

    return result;
}

#[derive(Clone)]
#[derive(Debug)]
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

#[derive(Clone)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

#[derive(Debug)]
enum ProgramEndReason {
    EndOfProgram,
    Loop,
    InvalidProgramCounter,
}

struct ProgramEnd {
    end_reason: ProgramEndReason,
    accumulator: i64,
}

impl std::fmt::Debug for ProgramEnd {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("ProgramEnd")
            .field("end reason", &self.end_reason)
            .field("accumulator", &self.accumulator)
            .finish()
    }
}

fn fix_and_detect_loop(input: &str) -> i64 {
    let mut instructions = parse_program(input);
    for index in 0..instructions.len() - 1 {
        {
            let mut instruction = &mut instructions[index];
            match instruction.operation {
                Operation::Acc => continue,
                Operation::Jmp => instruction.operation = Operation::Nop,
                Operation::Nop => instruction.operation = Operation::Jmp,
            }
        }

        let result = run_program(&instructions);
        match result.end_reason {
            ProgramEndReason::EndOfProgram => return result.accumulator,
            ProgramEndReason::InvalidProgramCounter => panic!("Unexpected end of program"),
            ProgramEndReason::Loop => {},
        }

        let mut instruction = &mut instructions[index];
        match instruction.operation {
            Operation::Acc => panic!(format!("Unexpected operation after program end '{:?}'", instruction.operation)),
            Operation::Jmp => instruction.operation = Operation::Nop,
            Operation::Nop => instruction.operation = Operation::Jmp,
        }
    }

    return -1;
}

fn detect_loop_input(input: &str) -> i64 {
    let instructions = parse_program(input);
    let result = run_program(&instructions);
    
    match result.end_reason {
        ProgramEndReason::Loop => {
            return result.accumulator;
        }
        _ => {
            panic!(format!("Loop not detected, {:?}", result));
        }
    }
}

fn run_program(instructions: &Vec<Instruction>) -> ProgramEnd {
    let mut accumulator: i64 = 0;
    let mut program_counter: i64 = 0;
    let instructions_count = instructions.len() as i64;

    let mut instructions: Vec<(&Instruction, bool)> = instructions.iter().map(|ins| (ins, false)).collect();
    loop {
        if program_counter < 0 || program_counter > instructions_count {
            return ProgramEnd {
                end_reason: ProgramEndReason::InvalidProgramCounter,
                accumulator: accumulator,
            }
        } else if program_counter == instructions_count {
            return ProgramEnd {
                end_reason: ProgramEndReason::EndOfProgram,
                accumulator: accumulator,
            }
        }

        let mut instruction = &mut instructions[program_counter as usize];

        if instruction.1 {
            return ProgramEnd {
                end_reason: ProgramEndReason::Loop,
                accumulator: accumulator,
            }
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
        let result = detect_loop_input(input);

        assert_eq!(5, result);
    }

    #[test]
    fn input_detect_loop() {
        let input = get_challenge_input();
        let result = detect_loop_input(input);

        assert_eq!(1262, result);
    }

    #[test]
    fn example_fix_and_detect_loop() {
        let input = get_example();
        let result = fix_and_detect_loop(input);

        assert_eq!(8, result);
    }

    #[test]
    fn input_fix_and_detect_loop() {
        let input = get_challenge_input();
        let result = fix_and_detect_loop(input);

        assert_eq!(1643, result);
    }
}
