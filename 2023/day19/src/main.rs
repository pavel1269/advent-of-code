const PARTS: usize = 4;
const TARGET_START: &str = "in";
const TARGET_ACCEPT: &str = "A";
const TARGET_REJECT: &str = "R";

fn main() {
    let input = get_input();
    let result_part1 = part1(&input);
    println!("Part1: {}", result_part1);
    let result_part2 = part2(&input);
    println!("Part2: {}", result_part2);
}

fn part1(input: &str) -> usize {
    let (rules, parts) = parse_input(input);

    let mut sum = 0;
    for part in parts.iter() {
        if rules.is_accepted(part) {
            sum += part.sum();
        }
    }

    return sum;
}

fn part2(input: &str) -> usize {
    let (rules, _) = parse_input(input);
    let mut map = Mapping::init(1, 4000);
    map.apply_rules(&rules);
    let result = map.sum_for(&TARGET_ACCEPT.to_string());
    return result;
}

#[derive(Debug)]
struct Mapping {
    map: Vec<RangeMapping>,
}

impl Mapping {
    fn sum_for(&self, target: &String) -> usize {
        self.map.iter().map(|map| map.sum_for(target)).sum()
    }

    fn apply_rules(&mut self, rules: &Rules) {
        while !self.is_done() {
            let index = self.find_first_not_done();
            let mut mapping = self.map.get(index).cloned().unwrap();
            self.map.remove(index);
            let rule = rules.get_rule(&mapping.target);
            let mappings = mapping.apply(rule);
            for mapping in mappings.into_iter().rev() {
                self.map.insert(index, mapping);
            }
        }
    }

    fn find_first_not_done(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .filter(|(_, map)| !map.is_done())
            .map(|(index, _)| index)
            .next()
            .unwrap()
    }

    fn is_done(&self) -> bool {
        self.map.iter().all(|map| map.is_done())
    }

    fn init(from: usize, to: usize) -> Self {
        let map = vec![RangeMapping::init(from, to)];
        let result = Self { map };
        return result;
    }
}

#[derive(Debug, Clone)]
struct RangeMapping {
    map: [Range; PARTS],
    target: String,
}

impl RangeMapping {
    fn sum_for(&self, target: &String) -> usize {
        if &self.target == target {
            return self.map.iter().map(|range| range.to.abs_diff(range.from) + 1).product();
        }
        return 0;
    }

    fn apply(&mut self, rule: &Rule) -> Vec<Self> {
        let mut result = Vec::new();
        let mut processing = Some(self.clone());
        for rule in rule.rules.iter() {
            if let Some(processing_map) = &processing {
                let (map1, map2) = processing_map.cut(rule);
                let mut processing_set = false;
                if let Some(map) = map1 {
                    if map.target == self.target {
                        processing = Some(map);
                        processing_set = true;
                    } else {
                        result.push(map);
                    }
                }
                if let Some(map) = map2 {
                    if map.target == self.target {
                        assert!(!processing_set);
                        processing = Some(map);
                    } else {
                        result.push(map);
                    }
                }
            } else {
                break;
            }
        }

        if let Some(mut map) = processing {
            map.target = rule.else_name.clone();
            result.push(map);
        }

        return result;
    }

    fn cut(&self, rule: &RuleCondition) -> (Option<RangeMapping>, Option<RangeMapping>) {
        let (index, amount) = rule
            .amount
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, amount)| amount.is_some())
            .map(|(index, amount)| (index, amount.unwrap()))
            .next()
            .unwrap();
        let (range1, range2) = self.map[index].cut(rule.op, amount);

        let map1 = self.apply_cut(range1, index, &rule.target, rule.op == RuleOperator::Less);
        let map2 = self.apply_cut(range2, index, &rule.target, rule.op == RuleOperator::More);

        return (map1, map2);
    }

    fn apply_cut(
        &self,
        cut_range: Option<Range>,
        index: usize,
        rule_target: &String,
        use_rule_target: bool,
    ) -> Option<RangeMapping> {
        if let Some(range) = cut_range {
            let target = if use_rule_target {
                rule_target.clone()
            } else {
                self.target.clone()
            };
            let mut map = self.map.clone();
            map[index] = range;

            Some(Self { map, target })
        } else {
            None
        }
    }

    fn is_done(&self) -> bool {
        self.target == TARGET_ACCEPT || self.target == TARGET_REJECT
    }

    fn init(from: usize, to: usize) -> Self {
        let map: [Range; PARTS] = vec![Range { from, to }; PARTS].try_into().unwrap();
        let result = TARGET_START.to_string();
        let result = Self {
            map,
            target: result,
        };
        return result;
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    from: usize,
    to: usize,
}

impl Range {
    fn cut(&self, op: RuleOperator, amount: usize) -> (Option<Self>, Option<Self>) {
        let range1_to = match op {
            RuleOperator::Less => amount - 1,
            RuleOperator::More => amount,
        };
        if self.to <= range1_to {
            return (Some(self.clone()), None);
        } else if self.from > range1_to {
            return (None, Some(self.clone()));
        } else {
            return (
                Some(Self {
                    from: self.from,
                    to: range1_to,
                }),
                Some(Self {
                    from: range1_to + 1,
                    to: self.to,
                }),
            );
        }
    }
}

struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    fn get_rule(&self, name: &String) -> &Rule {
        self.rules
            .iter()
            .filter(|rule| &rule.name == name)
            .next()
            .unwrap()
    }

    fn is_accepted(&self, part: &Part) -> bool {
        let mut state = TARGET_START.to_string();
        while state != TARGET_ACCEPT && state != TARGET_REJECT {
            let rule = self.get_rule(&state);
            state = rule.get_next_state(part);
        }
        return state == TARGET_ACCEPT;
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
struct RuleCondition {
    op: RuleOperator,
    amount: [Option<usize>; PARTS],
    target: String,
}

impl RuleCondition {
    fn applies(&self, part: &Part) -> bool {
        let (index, amount) = self
            .amount
            .iter()
            .enumerate()
            .filter(|(_, amount)| amount.is_some())
            .map(|(index, amount)| (index, amount.unwrap()))
            .next()
            .unwrap();
        return self.op.evaluate(amount, part.amounts[index]);
    }

    fn from(str: &str) -> Self {
        let regex = regex::Regex::new(r"^([xmas])([><])(\d+):([a-zA-Z]+)$").unwrap();
        let captures = regex.captures(str).unwrap();
        let gift = captures[1].chars().next().unwrap();
        let op = RuleOperator::from(captures[2].chars().next().unwrap()).unwrap();
        let mut amount = [None, None, None, None];
        let amount_number = captures[3].parse().unwrap();
        let target = captures[4].to_string();

        let index = match gift {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!(),
        };
        amount[index] = Some(amount_number);

        let result = Self { op, amount, target };
        return result;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    amounts: [usize; PARTS],
}

impl Part {
    fn sum(&self) -> usize {
        self.amounts.iter().sum()
    }

    fn from(str: &str) -> Self {
        let amounts: [usize; PARTS] = str
            .get(1..str.len() - 1)
            .unwrap()
            .split(',')
            .map(|split| split.split('=').skip(1).next().unwrap().parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let result = Self { amounts };
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

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = part2(&input);
        assert_eq!(result, 167409079868000);
    }
}
