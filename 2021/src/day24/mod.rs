use std::collections::{HashMap, HashSet};

pub fn get_solution_part1() -> i64 {
    return find_solution().1;
}

pub fn get_solution_part2() -> i64 {
    return find_solution().0;
}

fn find_solution() -> (i64, i64) {
    // i[3] + 8 = i[4]  -> 1
    // i[6] - 2 = i[7]  -> 3 - 9
    // i[8] + 7 = i[9]  -> 1 - 2
    // i[5]     = i[10] -> 1 - 9
    // i[2] - 5 = i[11] -> 6 - 9
    // i[1] + 2 = i[12] -> 1 - 7
    // i[0] - 4 = i[13] -> 5 - 9

    let min = 51619131181131;
    let max = 97919997299495;
    (min, max)
}

#[derive(Debug, PartialEq)]
enum Value {
    Variable(usize),
    Number(i64),
}

#[derive(Debug)]
enum Instructions {
    Inp(usize),
    Add(usize, Value),
    Mul(usize, Value),
    Div(usize, Value),
    Mod(usize, Value),
    Eql(usize, Value),
}

#[allow(dead_code)]
fn process_instructions(input: &str) {
    let instructions = parse_input(input);

    let mut states: HashMap<[i64; 4], Vec<HashSet<u8>>> = HashMap::new();
    states.insert([0, 0, 0, 0], vec![HashSet::new(); 14]);
    let mut input_index = 0;
    for (_index, instruction) in instructions.iter().enumerate() {
        println!("[{} / {}]: {}", _index, instructions.len(), states.len());
        println!("instruction: {:?}", instruction);
        println!("states: {:?}", states);
        println!();
        let mut states_new: HashMap<[i64; 4], Vec<HashSet<u8>>> = HashMap::new();
        match instruction {
            Instructions::Inp(register) => {
                for (registers, inputs) in states.iter() {
                    // Optimization based on how its calculated
                    if registers[3] != 0 {
                        continue;
                    }

                    for result in 1..10 {
                        let mut registers = registers.clone();
                        *registers.get_mut(*register).unwrap() = result;

                        let inputs_new = states_new
                            .entry(registers)
                            .or_insert(vec![HashSet::new(); 14]);

                        for (a, b) in inputs_new.iter_mut().zip(inputs) {
                            a.extend(b);
                        }
                        inputs_new
                            .get_mut(input_index)
                            .unwrap()
                            .insert(result as u8);
                    }
                }
                input_index += 1;
            }
            Instructions::Add(register, value) => {
                for (registers, inputs) in states.iter() {
                    let op1 = registers[*register];
                    let op2 = if let Value::Number(number) = value {
                        *number
                    } else if let Value::Variable(register2) = value {
                        registers[*register2]
                    } else {
                        panic!()
                    };
                    let mut registers = registers.clone();
                    *registers.get_mut(*register).unwrap() = op1 + op2;

                    for (a, b) in states_new
                        .entry(registers)
                        .or_insert(vec![HashSet::new(); 14])
                        .iter_mut()
                        .zip(inputs)
                    {
                        a.extend(b);
                    }
                }
            }
            Instructions::Mul(register, value) => {
                for (registers, inputs) in states.iter() {
                    let op1 = registers[*register];
                    let op2 = if let Value::Number(number) = value {
                        *number
                    } else if let Value::Variable(register2) = value {
                        registers[*register2]
                    } else {
                        panic!()
                    };
                    let mut registers = registers.clone();
                    *registers.get_mut(*register).unwrap() = op1 * op2;

                    for (a, b) in states_new
                        .entry(registers)
                        .or_insert(vec![HashSet::new(); 14])
                        .iter_mut()
                        .zip(inputs)
                    {
                        a.extend(b);
                    }
                }
            }
            Instructions::Div(register, value) => {
                for (registers, inputs) in states.iter() {
                    let op1 = registers[*register];
                    let op2 = if let Value::Number(number) = value {
                        *number
                    } else if let Value::Variable(register2) = value {
                        registers[*register2]
                    } else {
                        panic!()
                    };
                    let mut registers = registers.clone();
                    *registers.get_mut(*register).unwrap() = op1 / op2;

                    for (a, b) in states_new
                        .entry(registers)
                        .or_insert(vec![HashSet::new(); 14])
                        .iter_mut()
                        .zip(inputs)
                    {
                        a.extend(b);
                    }
                }
            }
            Instructions::Mod(register, value) => {
                for (registers, inputs) in states.iter() {
                    let op1 = registers[*register];
                    let op2 = if let Value::Number(number) = value {
                        *number
                    } else if let Value::Variable(register2) = value {
                        registers[*register2]
                    } else {
                        panic!()
                    };
                    let mut registers = registers.clone();
                    *registers.get_mut(*register).unwrap() = op1 % op2;

                    for (a, b) in states_new
                        .entry(registers)
                        .or_insert(vec![HashSet::new(); 14])
                        .iter_mut()
                        .zip(inputs)
                    {
                        a.extend(b);
                    }
                }
            }
            Instructions::Eql(register, value) => {
                for (registers, inputs) in states.iter() {
                    let op1 = registers[*register];
                    let op2 = if let Value::Number(number) = value {
                        *number
                    } else if let Value::Variable(register2) = value {
                        registers[*register2]
                    } else {
                        panic!()
                    };
                    let mut registers = registers.clone();
                    *registers.get_mut(*register).unwrap() = if op1 == op2 { 1 } else { 0 };

                    for (a, b) in states_new
                        .entry(registers)
                        .or_insert(vec![HashSet::new(); 14])
                        .iter_mut()
                        .zip(inputs)
                    {
                        a.extend(b);
                    }
                }
            }
        };

        states = states_new;
    }

    println!("states len: {}", states.len());
    println!("states: {:?}", states);

    todo!();
}

fn parse_input(input: &str) -> Vec<Instructions> {
    input
        .lines()
        .map(|input| {
            let (operation, rest) = input.split_at(3);
            let operand1 = (rest.chars().nth(1).unwrap() as u8 - 'w' as u8) as usize;

            if operation == "inp" {
                return Instructions::Inp(operand1);
            }

            let (_, operand2) = rest.split_at(3);
            let operand2_val = operand2.chars().nth(0).unwrap() as u8;
            let operand2 = if operand2_val >= 'w' as u8 && operand2_val <= 'z' as u8 {
                Value::Variable((operand2_val - 'w' as u8) as usize)
            } else {
                Value::Number(operand2.parse().unwrap())
            };

            match operation {
                "add" => Instructions::Add(operand1, operand2),
                "mul" => Instructions::Mul(operand1, operand2),
                "div" => Instructions::Div(operand1, operand2),
                "mod" => Instructions::Mod(operand1, operand2),
                "eql" => Instructions::Eql(operand1, operand2),
                _ => todo!(),
            }
        })
        .collect()
}

#[allow(dead_code)]
fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(97919997299495, get_solution_part1());
    }

    #[test]
    fn part2_test() {
        assert_eq!(51619131181131, get_solution_part2());
    }
}
