use std::collections::HashMap;

fn main() {
    let input = get_input();
    let map = parse_input(input);

    let result_part1 = navigate(&map);
    println!("Part1: {}", result_part1);

    let result_part2 = navigate_ghosts_cycles(&map);
    println!("Part2: {}", result_part2);
}

fn navigate_ghosts_cycles(map: &Map) -> u64 {
    let positions: Vec<String> = map.network.points.keys().filter(|point| point.ends_with('A')).cloned().collect();
    let targets: Vec<String> = map.network.points.keys().filter(|point| point.ends_with('Z')).cloned().collect();
    let mut lcm = 1;
    for position in positions {
        let result = navigate_point(map, &position, &targets, 0);
        lcm = num::integer::lcm(lcm, result as u64);
    }

    return lcm;
}

fn navigate(map: &Map) -> u32 {
    let mut position = "AAA".to_string();
    let target = "ZZZ".to_string();

    let mut instruction_index = 0;
    let mut steps = 0;
    while target != position {
        position = advance_on_map(map, &position, instruction_index);
        instruction_index = increase_with_max(instruction_index, map.instructions_right.len());
        steps += 1;
    }

    return steps;
}

fn navigate_point(map: &Map, position: &String, targets: &Vec<String>, instruction_index: usize) -> u32 {
    let mut position = position.clone();
    let mut instruction_index = instruction_index % map.instructions_right.len();
    let mut steps = 0;
    while steps == 0 || !targets.contains(&position) {
        position = advance_on_map(map, &position, instruction_index);
        instruction_index = increase_with_max(instruction_index, map.instructions_right.len());
        steps += 1;
    }

    return steps;
}

fn advance_on_map(map: &Map, position: &String, instruction_index: usize) -> String {
    let instruction_right = map.instructions_right[instruction_index];
    let crossroad = &map.network.points[position];
    let position = if instruction_right {
        crossroad.1.clone()
    } else {
        crossroad.0.clone()
    };
    return position;
}

fn increase_with_max(number: usize, max: usize) -> usize {
    let mut number = number + 1;
    if number >= max {
        number = 0;
    }
    return number;
}

#[derive(Debug)]
struct Map {
    instructions_right: Vec<bool>,
    network: Network,
}

#[derive(Debug)]
struct Network {
    points: HashMap<String, (String, String)>,
}

fn parse_input(input: &str) -> Map {
    let mut lines_iter = input.lines();
    let instructions_string = lines_iter.next().unwrap();
    let instructions_right = instructions_string.chars().map(|char| char == 'R').collect();

    let regex = regex::Regex::new(r"^(.+) = \((.+), (.+)\)$").unwrap();
    let mut points = HashMap::new();
    while let Some(line) = lines_iter.next() {
        if line.len() == 0 {
            continue;
        }
        let captures = regex.captures(line).unwrap();
        points.insert(captures[1].to_string(), (captures[2].to_string(), captures[3].to_string()));
    }

    let network = Network {
        points,
    };
    let map = Map {
        instructions_right,
        network,
    };
    return map;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example1_part1_input() -> &'static str {
        include_str!("./example1_part1.txt")
    }

    fn get_example2_part1_input() -> &'static str {
        include_str!("./example2_part1.txt")
    }

    #[test]
    fn part1_example1() {
        let input = get_example1_part1_input();
        let map = parse_input(input);

        let result = navigate(&map);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_example2() {
        let input = get_example2_part1_input();
        let map = parse_input(input);

        let result = navigate(&map);
        assert_eq!(result, 6);
    }

    fn get_example_part2_input() -> &'static str {
        include_str!("./example_part2.txt")
    }

    #[test]
    fn part2_example() {
        let input = get_example_part2_input();
        let map = parse_input(input);

        let result = navigate_ghosts_cycles(&map);
        assert_eq!(result, 6);
    }
}
