use std::collections::{HashMap, HashSet};

type Point = (i64, i64, i64);
type Distance = Point;
type Scanner = HashSet<Point>;
const COMMON_BEACONS_COUNT: usize = 12;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = count_beacons(input);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = calculate_scanner_distance(input);
    return result;
}

fn calculate_scanner_distance(input: &str) -> i64 {
    let mut scanners = parse_input(input);
    let mut distances = Vec::new();
    while scanners.len() > 1 {
        match try_merge_scanners(&scanners) {
            Some((base_scanner_index, merged_scanner_index, distance, merged_scanner)) => {
                distances.push(distance);
                scanners.remove(merged_scanner_index);
                *scanners.get_mut(base_scanner_index).unwrap() = merged_scanner;
            },
            None => panic!(),
        }
    }

    let mut distance_max = 0;
    for (index, point1) in distances.iter().copied().enumerate() {
        for point2 in distances.iter().skip(index + 1).copied() {
            let distance = (point1.0 - point2.0).abs() + (point1.1 - point2.1).abs() + (point1.2 - point2.2).abs();
            if distance > distance_max {
                distance_max = distance;
            }
        }
    }

    return distance_max;
}

fn count_beacons(input: &str) -> i64 {
    let mut scanners = parse_input(input);
    // println!("scanners: {}", scanners.len());

    while scanners.len() > 1 {
        match try_merge_scanners(&scanners) {
            Some((base_scanner_index, merged_scanner_index, _, merged_scanner)) => {
                // println!("  removing: {}", merged_scanner_index);
                scanners.remove(merged_scanner_index);
                *scanners.get_mut(base_scanner_index).unwrap() = merged_scanner;
            },
            None => panic!(),
        }
    }

    assert_eq!(1, scanners.len());
    let scanner = scanners.first().unwrap();
    return scanner.len() as i64;
}

fn try_merge_scanners(scanners: &Vec<Scanner>) -> Option<(usize, usize, Distance, Scanner)> {
    let mut merge_with = None;
    for (base_scanner_index, base_scanner) in scanners.iter().enumerate() {
        // println!("base scanner: {} ({}) vs ({})", base_scanner_index, base_scanner.len(), scanners.len());
        for (scanner2_index, scanner2) in
            scanners.iter().enumerate().skip(base_scanner_index + 1)
        {
            // println!("  testing: {}", scanner2_index);
            for scanner2 in rotate_scanner(&scanner2) {
                match try_match_points(base_scanner, &scanner2) {
                    None => continue,
                    Some(distance) => {
                        // println!("  match with {}", scanner2_index);
                        let mut moved_scanner = move_scanner(&scanner2, distance);
                        for point in base_scanner {
                            moved_scanner.insert(*point);
                        }
                        // println!("      {} + {} = {}", base_scanner.len(), scanner2.len(), moved_scanner.len());
                        merge_with = Some((base_scanner_index, scanner2_index, distance, moved_scanner));
                        break;
                    },
                }
            }

            if merge_with.is_some() {
                break;
            }
        }

        if merge_with.is_some() {
            break;
        }
    }
    return merge_with;
}

fn move_scanner(scanner: &Scanner, distance: Distance) -> Scanner {
    scanner
        .iter()
        .map(|(x, y, z)| (x + distance.0, y + distance.1, z + distance.2))
        .collect()
}

fn try_match_points(scanner1: &Scanner, scanner2: &Scanner) -> Option<Distance> {
    let mut distances_point: HashMap<Distance, HashSet<Point>> = HashMap::new();
    for point2 in scanner2.iter().copied() {
        for point1 in scanner1.iter().copied() {
            let diff = (
                point1.0 - point2.0,
                point1.1 - point2.1,
                point1.2 - point2.2,
            );
            distances_point.entry(diff).or_default().insert(point2);
        }
    }

    let matching_distances = distances_point
        .iter()
        .filter(|(_, points)| points.len() >= COMMON_BEACONS_COUNT)
        .map(|(distance, _)| *distance)
        .collect::<Vec<Distance>>();

    if matching_distances.is_empty() {
        return None;
    }

    let distance = matching_distances.first().unwrap();
    return Some(*distance);
}

fn rotate_scanner(scanner: &Scanner) -> Vec<Scanner> {
    // Rotations -> all axises can be swaped (6 combinations) and neither or exactly two are negative (4 combinations)
    // rotating by x upwads x,y,z-> x,z,y -> x,-y,-z -> x,-z,-y -> x,y,z
    // rotating by z towards right x,y,z-> y,-x,z -> -x,-y,z -> x,y,z
    // rotating by y towards right x,y,z-> -z,y,x ->

    let rotations_axis = [
        |(x, y, z): Point| (x, y, z),
        |(x, y, z): Point| (x, z, y),
        |(x, y, z): Point| (y, x, z),
        |(x, y, z): Point| (y, z, x),
        |(x, y, z): Point| (z, x, y),
        |(x, y, z): Point| (z, y, x),
    ];
    let rotations_rotate = [
        |(x, y, z): Point| (x, y, z),
        |(x, y, z): Point| (-x, -y, z),
        |(x, y, z): Point| (x, -y, -z),
        |(x, y, z): Point| (-x, y, -z),
        |(x, y, z): Point| (-x, -y, -z),
        |(x, y, z): Point| (-x, y, z),
        |(x, y, z): Point| (x, -y, z),
        |(x, y, z): Point| (x, y, -z),
    ];

    let mut rotations = Vec::new();
    for axis_rotation in rotations_axis {
        for rotate_rotation in rotations_rotate {
            let rotated = scanner
                .iter()
                .copied()
                .map(|point| axis_rotation(rotate_rotation(point)))
                .collect::<Scanner>();
            rotations.push(rotated);
        }
    }
    return rotations;
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut scanners = Vec::new();
    let mut scanner = Scanner::new();
    for line in input.lines() {
        if line.trim().len() < 1 {
            continue;
        }

        if line.starts_with("---") {
            if !scanner.is_empty() {
                scanners.push(scanner);
                scanner = Scanner::new();
            }
            continue;
        }

        let split = line.split(",").collect::<Vec<_>>();
        assert_eq!(3, split.len());

        let coord1 = split[0].parse().unwrap();
        let coord2 = split[1].parse().unwrap();
        let coord3 = split[2].parse().unwrap();
        scanner.insert((coord1, coord2, coord3));
    }

    if !scanner.is_empty() {
        scanners.push(scanner);
    }

    return scanners;
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
    fn parse_input_example_test() {
        let input = get_example_input();
        let result = parse_input(input);
        assert_eq!(5, result.len());
        let mut iter = result.iter();
        assert_eq!(25, iter.next().unwrap().len());
        assert_eq!(25, iter.next().unwrap().len());
        assert_eq!(26, iter.next().unwrap().len());
        assert_eq!(25, iter.next().unwrap().len());
        assert_eq!(26, iter.next().unwrap().len());
    }

    #[test]
    fn count_beacons_example_test() {
        let input = get_example_input();
        let result = count_beacons(input);
        assert_eq!(79, result);
    }

    #[test]
    fn count_beacons_input_test() {
        let result = get_solution_part1();
        assert_eq!(372, result);
    }

    #[test]
    fn calculate_scanner_distance_example_test() {
        let input = get_example_input();
        let result = calculate_scanner_distance(input);
        assert_eq!(3621, result);
    }

    #[test]
    fn calculate_scanner_input_test() {
        let result = get_solution_part2();
        assert_eq!(12241, result);
    }
}
