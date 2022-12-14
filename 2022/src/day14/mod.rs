use std::collections::{HashMap, HashSet};

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = fall_sand(input);
    return result.to_string();
}

fn fall_sand(input: &str) -> usize {
    let mut map = build_map(input);
    let max_y = *map.iter().map(|(_, y)| y.iter().max().unwrap()).max().unwrap();
    let mut sand_index = 0;
    while place_sand(&mut map, max_y) {
        sand_index += 1;
    }

    return sand_index;
}

fn place_sand(map: &mut HashMap<usize, HashSet<usize>>, max_y: usize) -> bool {
    let mut x = 500;
    let mut y = 0;

    while y <= max_y {
        if can_sand_move_down(map, x, y) {}
        else if can_sand_move_down_left(map, x, y) {
            x -= 1;
        }
        else if can_sand_move_down_right(map, x, y) {
            x += 1;
        }
        else {
            map.entry(x)
                .or_default()
                .insert(y);
            return true;
        }
        y += 1;
    }

    return false;
}

fn can_sand_move_down(map: &mut HashMap<usize, HashSet<usize>>, x: usize, y: usize) -> bool {
    match map.get(&x) {
        Some(column) => !column.contains(&(y + 1)),
        None => true,
    }
}

fn can_sand_move_down_left(map: &mut HashMap<usize, HashSet<usize>>, x: usize, y: usize) -> bool {
    can_sand_move_down(map, x - 1, y)
}

fn can_sand_move_down_right(map: &mut HashMap<usize, HashSet<usize>>, x: usize, y: usize) -> bool {
    can_sand_move_down(map, x + 1, y)
}

fn build_map(input: &str) -> HashMap<usize, HashSet<usize>> {
    let lines = parse_input(input);
    let mut map = HashMap::new();
    for line in lines.iter() {
        let mut previous_point = None;
        for point in line.iter().cloned() {
            if previous_point == None {
                previous_point = Some(point);
                continue;
            }

            draw_to_map(&mut map, previous_point.unwrap(), point);
            previous_point = Some(point);
        }
    }

    return map;
}

fn draw_to_map(map: &mut HashMap<usize, HashSet<usize>>, from: (usize, usize), to: (usize, usize)) {
    if from.0.abs_diff(to.0) > 0 {
        for x in from.0.min(to.0)..from.0.max(to.0) + 1 {
            map.entry(x)
                .or_default()
                .insert(from.1);
        }
    }
    else {
        for y in from.1.min(to.1)..from.1.max(to.1) + 1 {
            map.entry(from.0)
                .or_default()
                .insert(y);
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<(usize, usize)>> {
    let mut lines = Vec::new();
    for row in input.lines() {
        let mut points = Vec::new();
        for point in row.split(" -> ") {
            let mut point_split = point.split(",");
            let x = point_split.next().unwrap().parse().unwrap();
            let y = point_split.next().unwrap().parse().unwrap();
            points.push((x, y));
        }
        lines.push(points);
    }
    return lines;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = fall_sand(input);

        assert_eq!(result, 24);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "728");
    }
}
