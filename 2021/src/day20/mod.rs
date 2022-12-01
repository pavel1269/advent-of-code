pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = process_image(input, 2);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = process_image(input, 50);
    return result;
}

fn process_image(input: &str, iterations: usize) -> i64 {
    let (mut map, rules) = parse_input(input, iterations);
    let mut default = false;

    for _ in 0..iterations {
        let mut map_new = vec![vec![false; map.len()]; map.len()];
        for (row_index, row) in map.iter().enumerate() {
            for (column_index, _) in row.iter().enumerate() {
                let value = get_value(row_index as i32, column_index as i32, &map, default);
                let value = rules[value];
                if value {
                    map_new[row_index][column_index] = true;
                }
            }
        }

        if default {
            if !rules[0b111111111] {
                default = false;
            }
        } else {
            if rules[0] {
                default = true;
            }
        }

        map = map_new;
    }

    // print!("default: {}", default);
    // print_map(&map);

    let result = sum_points(&map);
    return result;
}

fn sum_points(map: &Vec<Vec<bool>>) -> i64 {
    let mut sum = 0;
    for row in map.iter() {
        for value in row.iter() {
            if *value {
                sum += 1;
            }
        }
    }
    return sum;
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<bool>>) {
    println!("map:");
    for row in map.iter() {
        for value in row.iter() {
            if *value {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn get_value(row: i32, column: i32, map: &Vec<Vec<bool>>, default: bool) -> usize {
    let mut number = 0;

    for row in row - 1..row + 2 {
        for column in column - 1..column + 2 {
            let value = if column < 0 || row < 0 || column as usize >= map.len() || row as usize >= map.len() {
                default
            } else {
                map[row as usize][column as usize]
            };

            number <<= 1;
            if value {
                number += 1;
            }
        }
    }

    return number;
}

fn parse_input(input: &str, iterations: usize) -> (Vec<Vec<bool>>, Vec<bool>) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap().trim();
    let mut rules = Vec::with_capacity(first_line.len());
    for char in first_line.chars() {
        match char {
            '.' => rules.push(false),
            '#' => rules.push(true),
            _ => panic!(),
        }
    }

    let mut map = Vec::new();
    let mut size = None;
    for line in lines {
        if line.len() < 2 {
            continue;
        }

        let mut map_line = vec![false; line.len() + iterations * 2];
        for (index, char) in line.chars().enumerate() {
            match char {
                '.' => continue,
                '#' => *map_line.get_mut(index + iterations).unwrap() = true,
                _ => panic!(),
            }
        }

        if map.len() == 0 {
            size = Some(map_line.len());
            for _ in 0..iterations {
                map.push(vec![false; size.unwrap()]);
            }
        }
        map.push(map_line);
    }

    for _ in 0..iterations {
        map.push(vec![false; size.unwrap()]);
    }

    return (map, rules);
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return include_str!("./example.txt");
    }

    #[test]
    fn process_image_example_1_iteration_test() {
        let input = get_example_input();
        let result = process_image(input, 1);
        assert_eq!(24, result);
    }

    #[test]
    fn process_image_example_2_iteration_test() {
        let input = get_example_input();
        let result = process_image(input, 2);
        assert_eq!(35, result);
    }

    #[test]
    fn part1_test() {
        let result = get_solution_part1();
        assert_eq!(5663, result);
    }

    #[test]
    fn process_image_example_50_iteration_test() {
        let input = get_example_input();
        let result = process_image(input, 50);
        assert_eq!(3351, result);
    }

    #[test]
    fn part2_test() {
        let result = get_solution_part2();
        assert_eq!(19638, result);
    }
}
