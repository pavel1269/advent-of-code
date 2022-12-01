use std::collections::HashMap;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = process_polymers(input, 10);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = process_polymers(input, 40);
    return result;
}

fn process_polymers(input: &str, iterations: usize) -> i64 {
    let (mut doubles, rules) = parse_input(input);

    for _ in 0..iterations {
        let mut new_doubles = HashMap::new();
        for (double, occurences) in doubles.iter() {
            if let Some((_, _, new_char)) = rules
                .iter()
                .copied()
                .find(|(rule_first, rule_second, _)| double.0 == *rule_first && double.1 == *rule_second)
            {
                *new_doubles.entry((double.0, new_char)).or_default() += *occurences;
                *new_doubles.entry((new_char, double.1)).or_default() += *occurences;
            } else {
                *new_doubles.entry(*double).or_default() += *occurences;
            }
        }

        doubles = new_doubles;
    }

    let mut counts = HashMap::new();
    for (double, occurenecs) in doubles.iter() {
        *counts.entry(double.0).or_default() += *occurenecs;
        *counts.entry(double.1).or_default() += *occurenecs;
    }
    counts.remove(&char::MAX);
    let mut counts = counts.iter().map(|(_, count)| *count).collect::<Vec<i64>>();
    counts.sort();
    return (counts.last().unwrap() - counts.first().unwrap()) / 2;
}

fn parse_input(input: &str) -> (HashMap<(char, char), i64>, Vec<(char, char, char)>) {
    let mut input_iter = input.lines();
    let template = input_iter.next().unwrap();

    let mut previous_char = None;
    let mut template_doubles = HashMap::new();
    for start_char in template.chars() {
        let double = if let Some(previous_char) = previous_char {
            (previous_char, start_char)
        } else {
            (char::MAX, start_char)
        };

        *template_doubles.entry(double).or_default() += 1;
        previous_char = Some(start_char);
    }
    template_doubles.insert((previous_char.unwrap(), char::MAX), 1);

    let mut rules = Vec::new();
    for line in input_iter {
        if line.len() <= 1 {
            continue;
        }

        let (input, output) = line.split_once(" -> ").unwrap();
        let output = output.chars().next().unwrap();
        let mut input = input.chars();
        let first = input.next().unwrap();
        let second = input.next().unwrap();
        rules.push((first, second, output))
    }

    return (template_doubles, rules);
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    }

    #[test]
    fn example_parsed_correctly() {
        let (doubles, rules) = parse_input(get_example_input());

        assert_eq!(5, doubles.len());
        assert_eq!(16, rules.len());
    }

    #[test]
    fn example_part1_correct_result() {
        let result = process_polymers(get_example_input(), 10);

        assert_eq!(1588, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(2590, result);
    }

    #[test]
    fn example_part2_correct_result() {
        let result = process_polymers(get_example_input(), 40);

        assert_eq!(2188189693529, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(2875665202438, result);
    }
}
