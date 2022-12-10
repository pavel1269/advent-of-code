use std::collections::HashMap;


pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = sum_signal_strengths(input);
    return result.to_string();
}

fn sum_signal_strengths(input: &str) -> i32 {
    let signals = execute(input);
    return signals.values().sum();
}

fn execute(input: &str) -> HashMap<usize, i32> {
    let instructions = parse_input(input);
    let mut pc = Pc::new();
    let mut signal_strengths: HashMap<usize, i32> = HashMap::new();

    let mut cycle_monitor = 20;
    for instruction in instructions.iter() {
        let pc_old = pc.clone();
        instruction.run(&mut pc);
        if pc.cycle >= cycle_monitor {
            signal_strengths.insert(cycle_monitor, i32::try_from(cycle_monitor).unwrap() * pc_old.x);
            cycle_monitor += 40;
        }
    }

    return signal_strengths;
}

#[derive(Clone, Copy)]
struct Pc {
    cycle: usize,
    x: i32,
}

impl Pc {
    fn new() -> Pc {
        Pc {
            cycle: 0,
            x: 1,
        }
    }
}

trait Instruction {
    fn run(&self, pc: &mut Pc);
}

struct InstructionNoop {}
impl Instruction for InstructionNoop {
    fn run(&self, pc: &mut Pc) {
        pc.cycle += 1;
    }
}

struct InstructionAddx {
    value: i32,
}
impl Instruction for InstructionAddx {
    fn run(&self, pc: &mut Pc) {
        pc.cycle += 2;
        pc.x += self.value;
    }
}

fn parse_input(input: &str) -> Vec<Box<dyn Instruction>> {
    let mut instructions: Vec<Box<dyn Instruction>>  = Vec::with_capacity(input.lines().count());
    let regex = regex::Regex::new(r"^addx (-?\d+)$").unwrap();
    for line in input.lines() {
        if line == "noop" {
            instructions.push(Box::new(InstructionNoop {}));
            continue;
        }
        match regex.captures(line) {
            None => panic!(),
            Some(captures) => {
                let value = captures[1].parse().unwrap();
                instructions.push(Box::new(InstructionAddx {
                    value: value,
                }));
            },
        }
    }

    return instructions;
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
        let result = sum_signal_strengths(input);

        assert_eq!(result, 13140);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "12740");
    }
}
