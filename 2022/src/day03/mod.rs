use std::collections::HashSet;

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = calculat_priority(input);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = calculat_priority_groups(input);
    return result.to_string();
}

fn calculat_priority_groups(input: &str) -> i32 {
    let offset_small = 'a' as i32;
    let offset_capital = 'A' as i32;
    let mut priority = 0;
    let mut lines_iter = input.lines();

    loop {
        let runsack1 = match lines_iter.next() {
            Some(runsack) => runsack,
            None => break,
        };
        let runsack2 = lines_iter.next().unwrap();
        let runsack3 = lines_iter.next().unwrap();

        let mut found = HashSet::new();
        runsack1.chars().for_each(|char| {
            if found.contains(&char) {
                return;
            }
            if runsack2.contains(char) && runsack3.contains(char) {
                found.insert(char);
                let char_ascii = char as i32;
                if char_ascii >= offset_small {
                    priority += char_ascii - offset_small + 1;
                }
                else {
                    priority += char_ascii - offset_capital + 27;
                }
            }
        });
    }
    return priority;
}

fn calculat_priority(input: &str) -> i32 {
    let offset_small = 'a' as i32;
    let offset_capital = 'A' as i32;
    let mut priority = 0;
    input.lines().for_each(|line| {
        let (compartment_first, compartment_second) = line.split_at(line.len() / 2);

        let mut found = HashSet::new();

        compartment_first.chars().for_each(|char| {
            if found.contains(&char) {
                return;
            }
            if compartment_second.contains(char) {
                found.insert(char);
                let char_ascii = char as i32;
                if char_ascii >= offset_small {
                    priority += char_ascii - offset_small + 1;
                }
                else {
                    priority += char_ascii - offset_capital + 27;
                }
            }
        })
    });
    return priority;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
    }
    
    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = calculat_priority(input);

        assert_eq!(result, 157);
    }
    
    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "7568");
    }
    
    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = calculat_priority_groups(input);

        assert_eq!(result, 70);
    }
}
