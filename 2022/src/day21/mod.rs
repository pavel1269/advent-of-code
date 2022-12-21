use std::{collections::HashMap, cell::RefCell};

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = get_calc_monkey(input);
    return result.to_string();
}

fn get_calc_monkey(input: &str) -> i64 {
    let mut monkeys = Monkeys::from(input);
    monkeys.reduce();
    return monkeys.get_value(&"root".to_string());
}

struct Monkeys {
    monkeys: HashMap<String, RefCell<Monkey>>,
    names: Vec<String>,
}

impl Monkeys {
    fn reduce(&mut self) {
        let mut reduced = true;
        while reduced {
            reduced = false;
            for name in self.names.iter() {
                let mut monkey = self.monkeys.get(name).unwrap().borrow_mut();
                if monkey.number.is_some() {
                    continue;
                }

                let eq = monkey.equation.as_ref().unwrap();
                
                let op1 = self.monkeys.get(&eq.op1).unwrap().borrow();
                if op1.number.is_none() {
                    continue;
                }
                let op2 = self.monkeys.get(&eq.op2).unwrap().borrow();
                if op2.number.is_none() {
                    continue;
                }
                
                let op1 = op1.number.unwrap();
                let op2 = op2.number.unwrap();
                let result = eq.op.calc(op1, op2);
                monkey.number = Some(result);
                reduced = true;
            }
        }
    }

    fn get_value(&self, monkey: &String) -> i64 {
        self.monkeys[monkey].borrow().number.unwrap()
    }

    fn from(input: &str) -> Monkeys {
        let mut monkeys = Monkeys {
            monkeys: HashMap::new(),
            names: Vec::new(),
        };
        for row in input.lines() {
            let monkey = Monkey::from(row);
            monkeys.names.push(monkey.name.clone());
            monkeys.monkeys.insert(monkey.name.clone(), RefCell::from(monkey));
        }
        return monkeys;
    }
}

#[derive(Debug)]
struct Monkey {
    name: String,
    number: Option<i64>,
    equation: Option<Equation>,
}

impl Monkey {
    fn from(text: &str) -> Monkey {
        let number_monkey = Self::try_parse_number_monkey(text);
        if number_monkey.is_some() {
            return number_monkey.unwrap();
        }

        let eq_monkey = Self::try_parse_eq_monkey(text);
        return eq_monkey.unwrap();
    }

    fn try_parse_eq_monkey(text: &str) -> Option<Monkey> {
        let regex_equation = regex::Regex::new(r"^(\w{4}): (\w{4}) ([+-/*]) (\w{4})$").unwrap();
        let captures = regex_equation.captures(text);
        if captures.is_none() {
            return None;
        }
        let captures = captures.unwrap();

        let eq = Equation {
            op1: captures[2].to_string(),
            op: Operation::from(&captures[3]),
            op2: captures[4].to_string(),
        };
        let monkey = Monkey {
            name: captures[1].to_string(),
            number: None,
            equation: Some(eq),
        };
        return Some(monkey);
    }

    fn try_parse_number_monkey(text: &str) -> Option<Monkey> {
        let regex_number = regex::Regex::new(r"^(\w{4}): (\d+)$").unwrap();
        let captures = regex_number.captures(text);
        if captures.is_none() {
            return None;
        }
        let captures = captures.unwrap();

        let monkey = Monkey {
            name: captures[1].to_string(),
            number: Some(captures[2].parse().unwrap()),
            equation: None,
        };
        return Some(monkey);
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Substract,
    Multiply,
    Divide,
}

impl Operation {
    fn calc(&self, op1: i64, op2: i64) -> i64 {
        match self {
            Self::Add => op1 + op2,
            Self::Substract => op1 - op2,
            Self::Multiply => op1 * op2,
            Self::Divide => op1 / op2,
        }
    }

    fn from(text: &str) -> Operation {
        match text {
            "+" => Self::Add,
            "-" => Self::Substract,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Equation {
    op1: String,
    op: Operation,
    op2: String,
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = get_calc_monkey(input);

        assert_eq!(result, 152);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "158731561459602");
    }
}
