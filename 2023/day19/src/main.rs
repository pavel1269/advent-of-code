fn main() {
    let input = get_input();
    let result_part1 = part1(&input);
    println!("Part1: {}", result_part1);
}

fn part1(input: &str) -> usize {
    let mut sum = 0;
    let (rules, parts) = parse_input(input);

    for part in parts.iter() {
        if rules.is_accepted(part) {
            sum += part.sum();
        }
    }

    return sum;
}

struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    fn is_accepted(&self, part: &Part) -> bool {
        let mut state = "in".to_string();
        let state_accepted = "A".to_string();
        let state_rejected = "R".to_string();
        while state != state_accepted && state != state_rejected {
            let rule = self.rules.iter().filter(|rule| rule.name == state).next().unwrap();
            state = rule.get_next_state(part);
        }
        return state == state_accepted;
    }
}

struct Rule {
    name: String,
    rules: Vec<RuleCondition>,
    else_name: String,
}

impl Rule {
    fn get_next_state(&self, part: &Part) -> String {
        for rule in self.rules.iter() {
            if rule.applies(part) {
                return rule.target.clone();
            }
        }
        return self.else_name.clone();
    }

    fn from(str: &str) -> Self {
        let regex = regex::Regex::new(r"^([a-zA-Z]+)\{(.*),([a-zA-Z]+)\}$").unwrap();
        let captures = regex.captures(str).unwrap();
        let name = captures[1].to_string();
        let rules = captures[2]
            .split(',')
            .map(|str| RuleCondition::from(str))
            .collect();
        let else_name = captures[3].to_string();

        let result = Self {
            name,
            rules,
            else_name,
        };
        return result;
    }
}

struct RuleCondition {
    op: RuleOperator,
    amount_x: Option<usize>,
    amount_m: Option<usize>,
    amount_a: Option<usize>,
    amount_s: Option<usize>,
    target: String,
}

impl RuleCondition {
    fn applies(&self, part: &Part) -> bool {
        if let Some(amount) = self.amount_x {
            return self.op.evaluate(amount, part.x);
        }
        if let Some(amount) = self.amount_m {
            return self.op.evaluate(amount, part.m);
        }
        if let Some(amount) = self.amount_a {
            return self.op.evaluate(amount, part.a);
        }
        if let Some(amount) = self.amount_s {
            return self.op.evaluate(amount, part.s);
        }
        panic!()
    }

    fn from(str: &str) -> Self {
        let regex = regex::Regex::new(r"^([xmas])([><])(\d+):([a-zA-Z]+)$").unwrap();
        let captures = regex.captures(str).unwrap();
        let gift = captures[1].chars().next().unwrap();
        let op = RuleOperator::from(captures[2].chars().next().unwrap()).unwrap();
        let mut amount_x = None;
        let mut amount_m = None;
        let mut amount_a = None;
        let mut amount_s = None;
        let amount = captures[3].parse().unwrap();
        let target = captures[4].to_string();

        match gift {
            'x' => amount_x = Some(amount),
            'm' => amount_m = Some(amount),
            'a' => amount_a = Some(amount),
            's' => amount_s = Some(amount),
            _ => panic!(),
        }

        let result = Self {
            op,
            amount_x,
            amount_m,
            amount_a,
            amount_s,
            target,
        };
        return result;
    }
}

enum RuleOperator {
    Less,
    More,
}

impl RuleOperator {
    fn evaluate(&self, amount: usize, amount_part: usize) -> bool {
        match self {
            RuleOperator::Less => return amount > amount_part,
            RuleOperator::More => return amount < amount_part,
        }
    }

    fn from(char: char) -> Option<Self> {
        match char {
            '>' => Some(Self::More),
            '<' => Some(Self::Less),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn from(str: &str) -> Self {
        let amounts: Vec<usize> = str
            .get(1..str.len() - 1)
            .unwrap()
            .split(',')
            .map(|split| split.split('=').skip(1).next().unwrap().parse().unwrap())
            .collect();
        assert!(amounts.len() == 4);
        let result = Self {
            x: amounts[0],
            m: amounts[1],
            a: amounts[2],
            s: amounts[3],
        };
        return result;
    }
}

fn parse_input(input: &str) -> (Rules, Vec<Part>) {
    let rules: Vec<_> = input
        .lines()
        .take_while(|line| line.len() > 0)
        .map(|line| Rule::from(line))
        .collect();
    let rules = Rules { rules };
    let parts: Vec<_> = input
        .lines()
        .skip_while(|line| line.len() > 0)
        .skip(1)
        .map(|line| Part::from(line))
        .collect();
    return (rules, parts);
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
        let result = part1(&input);
        assert_eq!(result, 19114);
    }
}
