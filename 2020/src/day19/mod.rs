use regex::Regex;
use std::collections::HashMap;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = count_matches(input, false);

    return result;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let result = count_matches(input, true);

    return result;
}

fn modify_rules_loop(rules: &mut HashMap<usize, Rule>) {
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    rules.insert(8, Rule::RuleGroups(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::RuleGroups(vec![vec![42, 31], vec![42, 11, 31]]));
}

fn count_matches(input: &str, include_loop: bool) -> i64 {
    let mut input = parse_input(input);
    if include_loop {
        modify_rules_loop(&mut input.rules);
    }
    let regex = build_regex(&mut input.rules);
    let regex = Regex::new(regex.as_str()).unwrap();

    let mut matches: i64 = 0;
    // println!("{:?}", input.rules);
    for sth in input.inputs {
        // println!("'{}' match: {}", sth, regex.is_match(sth.as_str()));
        if regex.is_match(sth.as_str()) {
            matches += 1;
        }
    }

    return matches;
}

fn build_regex(rules: &mut HashMap<usize, Rule>) -> String {
    let mut rules_to_rework = rules
        .iter()
        .filter(|rule| !rule.1.is_string())
        .map(|rule| (*rule.0, rule.1.clone()))
        .collect::<Vec<(usize, Rule)>>();

    while rules_to_rework.len() > 0 {
        // println!("{:?}", rules);
        let mut solved_rules: Vec<usize> = Vec::new();
        for (index_rework, (index_rule, rule)) in rules_to_rework.iter().enumerate() {
            if rule.is_string() {
                continue;
            }

            let mut all_string = true;
            for group in rule.get_groups() {
                if !all_string {
                    break;
                }
                for index_rule in group {
                    match rules.get(index_rule) {
                        Some(sub_rule) => {
                            if rule != sub_rule && !sub_rule.is_string() {
                                all_string = false;
                                break;
                            }
                        }
                        None => panic!(format!("Rule {} not found", index_rule)),
                    }
                }
            }

            if !all_string {
                continue;
            }

            let mut groups: Vec<String> = Vec::new();
            for group in rule.get_groups() {
                let mut string_rule: String = String::with_capacity(group.len());
                let mut used_groups = false;
                let mut string_rule_part: String = String::new();
                for index_sub_rule in group {
                    if index_sub_rule == index_rule {
                        if used_groups {
                            panic!();
                        }
                        used_groups = true;
                        string_rule_part = string_rule.clone();
                        string_rule.clear();
                    } else {
                        string_rule.push_str(rules[index_sub_rule].get_string());
                    }
                }

                if used_groups {
                    if string_rule.len() == 0 {
                        debug_assert!(string_rule_part.len() > 0);
                        string_rule = format!("({})+", string_rule_part);
                    } else if string_rule.len() > 0 && string_rule_part.len() > 0 {
                        let mut string_rule_parts: Vec<String> = vec![];
                        for times in 1..5 {
                            let mut immediate_part = String::new();
                            for _ in 0..times {
                                immediate_part.push_str(string_rule_part.as_str());
                            }
                            for _ in 0..times {
                                immediate_part.push_str(string_rule.as_str());
                            }
                            string_rule_parts.push(immediate_part);
                        }
                        string_rule = format!("({})", string_rule_parts.join("|"));
                    } else {
                        panic!();
                    }
                }

                groups.push(string_rule);
            }

            let string_rule = if groups.len() > 1 {
                let mut string_rule = String::from("(");
                string_rule.push_str(groups.join("|").as_str());
                string_rule.push(')');
                string_rule
            } else {
                groups.first().cloned().unwrap()
            };
            rules.insert(*index_rule, Rule::String(string_rule));
            solved_rules.push(index_rework);
            // println!("reworked rule {} / {}", index, rules_to_rework.len());
        }

        if solved_rules.len() == 0 {
            panic!(format!(
                "Not all rules solved, rules: {:?}",
                rules_to_rework
            ));
        }

        for index in solved_rules.iter().rev() {
            rules_to_rework.remove(*index);
        }
        solved_rules.clear();
    }

    let rule_0 = rules[&0].get_string();
    let mut final_rule = String::with_capacity(rule_0.len() + 2);
    final_rule.push('^');
    final_rule.push_str(rule_0);
    final_rule.push('$');

    return final_rule;
}

#[derive(Debug)]
struct Input {
    rules: HashMap<usize, Rule>,
    inputs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
enum Rule {
    String(String),
    RuleGroups(Vec<Vec<usize>>),
}

impl Rule {
    fn is_string(&self) -> bool {
        matches!(self, Rule::String(_))
    }

    fn get_string(&self) -> &String {
        match self {
            Rule::String(str) => str,
            _ => panic!(),
        }
    }

    fn get_groups(&self) -> &Vec<Vec<usize>> {
        match self {
            Rule::RuleGroups(groups) => groups,
            _ => panic!(),
        }
    }
}

fn parse_input(input: &str) -> Input {
    let rule_regex = Regex::new("^(\\d+): (.*)$").unwrap();
    let string_regex = Regex::new("^\"(\\w+)\"$").unwrap();
    let mut rules: HashMap<usize, Rule> = HashMap::new();
    let mut input_lines = input.lines();
    for line in &mut input_lines {
        let captures = rule_regex.captures(line);
        if captures.is_none() {
            break;
        }
        let captures = captures.unwrap();
        let index = captures[1].parse::<usize>().unwrap();

        match string_regex.captures(&captures[2]) {
            Some(captures) => {
                rules.insert(index, Rule::String(captures[1].to_string()));
            }
            None => {
                let mut groups: Vec<Vec<usize>> = Vec::new();
                for rule_group in captures[2].split("|") {
                    let mut group: Vec<usize> = Vec::new();
                    for rule in rule_group.trim().split(" ") {
                        group.push(rule.trim().parse::<usize>().unwrap());
                    }
                    groups.push(group);
                }
                rules.insert(index, Rule::RuleGroups(groups));
            }
        }
    }

    let mut inputs: Vec<String> = Vec::new();
    for line in &mut input_lines {
        inputs.push(String::from(line));
    }

    return Input {
        rules: rules,
        inputs: inputs,
    };
}

fn get_challenge_input() -> &'static str {
    include_str!("input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        include_str!("example.txt")
    }

    fn get_example2_input() -> &'static str {
        include_str!("example2.txt")
    }

    #[test]
    fn example_count_matches() {
        let input = get_example_input();
        let result = count_matches(input, false);

        assert_eq!(2, result);
    }

    #[test]
    fn input_parsed_rules_count() {
        let input = get_challenge_input();
        let result = parse_input(input);

        assert_eq!(133, result.rules.len(), "{:?}", result.rules.keys());
    }

    #[test]
    fn input_parsed_inputs_count() {
        let input = get_challenge_input();
        let result = parse_input(input);

        assert_eq!(355, result.inputs.len());
    }

    #[test]
    fn input_part1() {
        let result = get_part1_result();

        assert_eq!(203, result);
    }

    #[test]
    fn example2_count_matches() {
        let input = get_example2_input();
        let result = count_matches(input, false);

        assert_eq!(3, result);
    }

    #[test]
    fn example2_count_matches_with_loop() {
        let input = get_example2_input();
        let result = count_matches(input, true);

        assert_eq!(12, result);
    }
}
