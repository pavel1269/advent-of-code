use std::collections::HashMap;

fn main() {
    let input = get_input();
    let map = parse_input(input);
    
    let result_part1 = navigate(&map);
    println!("Part1: {}", result_part1);
}

fn navigate(map: &Map) -> u32 {
    let mut position = "AAA".to_string();
    let target = "ZZZ".to_string();

    let mut instruction_index = 0;
    let mut steps = 0;
    while target != position {
        let instruction_right = map.instructions_right[instruction_index];
        instruction_index += 1;
        if instruction_index >= map.instructions_right.len() {
            instruction_index = 0;
        }

        let crossroad = &map.network.points[&position];
        position = if instruction_right {
            crossroad.1.clone()
        } else {
            crossroad.0.clone()
        };
        steps += 1;
    }

    return steps;
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

    fn get_example1_input() -> &'static str {
        include_str!("./example1.txt")
    }

    fn get_example2_input() -> &'static str {
        include_str!("./example2.txt")
    }

    #[test]
    fn part1_example1() {
        let input = get_example1_input();
        let map = parse_input(input);

        let result = navigate(&map);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_example2() {
        let input = get_example2_input();
        let map = parse_input(input);

        let result = navigate(&map);
        assert_eq!(result, 6);
    }
}
