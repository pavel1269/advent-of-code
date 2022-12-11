
pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = calc_inspections(input);
    return result.to_string();
}

fn calc_inspections(input: &str) -> usize {
    let mut inspections = monkey_business(input);
    inspections.sort();

    let max = inspections.pop().unwrap();
    let max2 = inspections.pop().unwrap();

    return max * max2;
}

#[derive(Debug)]
enum MonkeyOps {
    Add,
    Mult,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: MonkeyOps,
    op_value: Option<usize>,
    test: usize,
    target_true: usize,
    target_false: usize,
}

impl Monkey {
    fn apply_worry_item(&self, item: usize) -> usize {
        let value = match self.op_value {
            None => item,
            Some(value) => value,
        };
        let item = match self.op {
            MonkeyOps::Add => item + value,
            MonkeyOps::Mult => item * value,
        };
        return item / 3;
    }

    fn get_target(&self, item: usize) -> usize {
        if item % self.test == 0 {
            self.target_true
        }
        else {
            self.target_false
        }
    }
}

fn monkey_business(input: &str) -> Vec<usize> {
    let mut monkeys = parse_input(input);
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..20 {
        for index_monkey in 0..monkeys.len() {
            for item in monkeys[index_monkey].items.clone().iter().copied() {
                let monkey = &mut monkeys[index_monkey];
                monkey.items.remove(0);
                inspections[index_monkey] += 1;
                let item = monkey.apply_worry_item(item);
                let target = monkey.get_target(item);
                monkeys[target].items.push(item);
            }
        }

        // for (monkey_index, monkey) in monkeys.iter().enumerate()
        // {
        //     println!("Monkey {}: {:?}", monkey_index, monkey.items);
        // }
        // println!();
    }
    return inspections;
}

fn parse_input(input: &str) -> Vec<Monkey> {
    let regex_items = regex::Regex::new(r"^  Starting items: (.*)$").unwrap();
    let regex_op = regex::Regex::new(r"^  Operation: new = old ([+*]) (\d+|old)$").unwrap();
    let regex_test = regex::Regex::new(r"^  Test: divisible by (\d+)$").unwrap();
    let regex_true = regex::Regex::new(r"^    If true: throw to monkey (\d+)$").unwrap();
    let regex_false = regex::Regex::new(r"^    If false: throw to monkey (\d+)$").unwrap();

    let mut monkeys = Vec::new();
    let mut input_iter = input.lines();
    while let Some(_) = input_iter.next() {
        let items_line = input_iter.next().unwrap();
        let op_line = input_iter.next().unwrap();
        let test_line = input_iter.next().unwrap();
        let true_line = input_iter.next().unwrap();
        let false_line = input_iter.next().unwrap();
        input_iter.next();

        let captures = regex_items.captures(items_line).unwrap();
        let items = captures[1].split(", ").map(|item| item.parse().unwrap()).collect();

        let captures = regex_op.captures(op_line).unwrap();
        let op = match &captures[1] {
            "*" => MonkeyOps::Mult,
            "+" => MonkeyOps::Add,
            _ => panic!(),
        };
        let op_value = if &captures[2] == "old" {
            None
        } else {
            Some(captures[2].parse().unwrap())
        };
        let captures = regex_test.captures(test_line).unwrap();
        let test = captures[1].parse().unwrap();
        let captures = regex_true.captures(true_line).unwrap();
        let true_test_target = captures[1].parse().unwrap();
        let captures = regex_false.captures(false_line).unwrap();
        let false_test_target = captures[1].parse().unwrap();

        monkeys.push(Monkey {
            items: items,
            op: op,
            op_value: op_value,
            test: test,
            target_true: true_test_target,
            target_false: false_test_target,
        });
    }

    return monkeys;
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
        let result = calc_inspections(input);

        assert_eq!(result, 10605);
    }
}
