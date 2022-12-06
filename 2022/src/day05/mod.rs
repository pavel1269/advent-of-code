
pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = get_last_crates(input);
    return result;
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = get_last_crates_multiple(input);
    return result;
}

fn get_last_crates_multiple(input: (&str, &str)) -> String {
    let (mut stacks, instructions) = parse_input(input);
    move_crates_multiple(&mut stacks, instructions);
    let mut result = String::new();
    for stack in stacks.iter_mut() {
        result.push(stack.pop().unwrap());
    }
    return result;
}

fn move_crates_multiple(stacks: &mut Vec<Vec<char>>, instructions: Vec<(usize, usize, usize)>) {
    for (from, to, count) in instructions.iter() {
        let mut tmp = Vec::with_capacity(*count);
        for _ in 0..*count {
            let top_crate = stacks[*from].pop().unwrap();
            tmp.push(top_crate);
        }
        for _ in 0..*count {
            let top_crate = tmp.pop().unwrap();
            stacks[*to].push(top_crate);
        };
    }
}

fn get_last_crates(input: (&str, &str)) -> String {
    let (mut stacks, instructions) = parse_input(input);
    move_crates(&mut stacks, instructions);
    let mut result = String::new();
    for stack in stacks.iter_mut() {
        result.push(stack.pop().unwrap());
    }
    return result;
}

fn move_crates(stacks: &mut Vec<Vec<char>>, instructions: Vec<(usize, usize, usize)>) {
    for (from, to, count) in instructions.iter() {
        for _ in 0..*count {
            let top_crate = stacks[*from].pop().unwrap();
            stacks[*to].push(top_crate);
        };
    }
}

fn parse_input((stacks, instructions): (&str, &str)) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let stacks = parse_stacks(stacks);
    let instructions = parse_instructions(instructions);
    return (stacks, instructions);
}

fn parse_stacks(stacks: &str) -> Vec<Vec<char>> {
    let mut stack_iter = stacks.lines().rev();
    let stack_line_1 = stack_iter.next().unwrap();
    let stack_count = stack_line_1.split(' ').filter(|s| s.len() > 0).collect::<Vec<&str>>().len();
    let mut stacks = vec![vec![]; stack_count];

    for stack_line in stack_iter {
        for index in 0..stack_count {
            let crate_index = 1 + index * 4;
            let crate_name = stack_line.chars().nth(crate_index).unwrap();

            if crate_name == ' ' {
                continue;
            }
            if crate_name < 'A' || crate_name > 'Z' {
                panic!("Failed to parse: '{}'", stack_line);
            }

            stacks[index].push(crate_name);
        }
    }

    return stacks;
}

fn parse_instructions(instructions_str: &str) -> Vec<(usize, usize, usize)> {
    let mut instructions = Vec::with_capacity(instructions_str.lines().count());
    let regex = regex::Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    instructions_str.lines().for_each(|line| {
        match regex.captures(line) {
            Some(captures) => {
                let count = captures[1].parse().unwrap();
                let from: usize = captures[2].parse().unwrap();
                let to: usize = captures[3].parse().unwrap();
                instructions.push((from - 1, to - 1, count));
            },
            None => panic!("Failed to parse: {}", line),
        }
    });
    return instructions;
}

fn get_input() -> (&'static str, &'static str) {
    (include_str!("./input-stacks.txt"), include_str!("./input-instructions.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> (&'static str, &'static str) {
        ("    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ",
        "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2")
    }
    
    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = get_last_crates(input);

        assert_eq!(result, "CMZ");
    }
    
    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "GRTSWNJHH");
    }
    
    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = get_last_crates_multiple(input);

        assert_eq!(result, "MCD");
    }
    
    #[test]
    fn part2_input() {
        let result = get_solution_part2();

        assert_eq!(result, "QLFQDBBHM");
    }
}
