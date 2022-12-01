use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = calculate_path_count(input, false);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = calculate_path_count(input, true);
    return result;
}

fn calculate_path_count(input: &str, can_revisit_small: bool) -> i64 {
    static START: &str = "start";
    static END: &str = "end";
    
    let input = parse_input(input);

    let mut small_caves: HashSet<&str> = HashSet::new();
    let mut possible_paths: HashMap<&str, Vec<&str>> = HashMap::new();
    let small_cave_regex: Regex = Regex::new("^[a-z]+$").unwrap();
    for (start, end) in input {
        if small_cave_regex.is_match(start) && !small_caves.contains(start) {
            small_caves.insert(start);
        }
        if small_cave_regex.is_match(end) && !small_caves.contains(end) {
            small_caves.insert(end);
        }

        if !possible_paths.contains_key(start) {
            possible_paths.insert(start, Vec::new());
        }
        if !possible_paths.contains_key(end) {
            possible_paths.insert(end, Vec::new());
        }

        possible_paths.get_mut(start).unwrap().push(end);
        possible_paths.get_mut(end).unwrap().push(start);
    }

    let mut paths = vec![(can_revisit_small, vec![START])];
    let mut final_paths = Vec::new();
    while paths.len() > 0 {
        let (can_revisit_small, current_path) = paths.pop().unwrap();
        let last_cave = *current_path.last().unwrap();

        if last_cave == END {
            // println!("final: {}", current_path.join(","));
            final_paths.push(current_path);
            continue;
        }

        for next_cave in &possible_paths[last_cave] {
            let mut can_revisit_small = can_revisit_small;
            if small_caves.contains(next_cave) && current_path.contains(next_cave) {
                if next_cave == &START {
                    continue;
                } else if can_revisit_small {
                    can_revisit_small = false;
                    // println!("can revisit = false (next: {})", next_cave);
                } else {
                    // println!("skipping: {},{}", current_path.join(","), next_cave);
                    continue;
                }
            }

            let mut new_path = current_path.clone();
            new_path.push(next_cave);
            // println!("next: {} (revisit: {})", new_path.join(","), can_revisit_small);
            paths.push((can_revisit_small, new_path));
        }
    }

    // println!();
    // for path in final_paths.iter() {
    //     println!("{}", path.join(","));
    // }

    return final_paths.len() as i64;
}

fn parse_input(intput: &str) -> Vec<(&str, &str)> {
    return intput
        .lines()
        .map(|line| line.trim().split_once('-').unwrap())
        .collect::<Vec<_>>();
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_1_input() -> &'static str {
        return "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    }

    fn get_example_2_input() -> &'static str {
        return "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    }

    fn get_example_3_input() -> &'static str {
        return "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
    }

    #[test]
    fn example1_part1_correct_result() {
        let result = calculate_path_count(get_example_1_input(), false);

        assert_eq!(10, result);
    }

    #[test]
    fn example2_part1_correct_result() {
        let result = calculate_path_count(get_example_2_input(), false);

        assert_eq!(19, result);
    }

    #[test]
    fn example3_part1_correct_result() {
        let result = calculate_path_count(get_example_3_input(), false);

        assert_eq!(226, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(3779, result);
    }

    #[test]
    fn example1_part2_correct_result() {
        let result = calculate_path_count(get_example_1_input(), true);

        assert_eq!(36, result);
    }

    #[test]
    fn example2_part2_correct_result() {
        let result = calculate_path_count(get_example_2_input(), true);

        assert_eq!(103, result);
    }

    #[test]
    fn example3_part2_correct_result() {
        let result = calculate_path_count(get_example_3_input(), true);

        assert_eq!(3509, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(96988, result);
    }
}
