use std::collections::HashSet;

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = coverage_at(input, 2000000);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = get_tuning_frequency(input, 4000000);
    return result.to_string();
}

fn get_tuning_frequency(input: &str, limit: i32) -> i64 {
    let result = locate_distress(input, limit);
    return result.x as i64 * 4000000 + result.y as i64;
}

fn locate_distress(input: &str, limit: i32) -> Point {
    let map = parse_input(input);
    for y in 0..limit + 1 {
        let x = map.get_next_uncovered(0, y);
        if x <= limit {
            return Point::from(x, y);
        }
    }
    panic!();
}

fn coverage_at(input: &str, check_coverage: i32) -> usize {
    let map = parse_input(input);
    let mut coverage = 0;
    for x in map.lowest_x - map.furthest_beacon..map.highest_x + map.furthest_beacon {
        let check = Point::from(x, check_coverage);
        if map.beacons.contains(&check) {
            continue;
        }
        if map.is_covered(&check) {
            coverage += 1;
        }
    }
    println!();
    return coverage;
}

struct Map {
    sensors: Vec<Sensor>,
    beacons: HashSet<Point>,
    furthest_beacon: i32,
    lowest_x: i32,
    highest_x: i32,
}

impl Map {
    fn get_next_uncovered(&self, x: i32, y: i32) -> i32 {
        // y == sy -> <x - distance, x + distance>
        // y == sy + 1 -> <x - distance + 1, x + distance - 1>
        // y == sy + ydiff -> <x - distance + ydiff, x + distance - ydiff>
        let ranges = self.sensors.iter()
            .map(|sensor| {
                let ydiff = sensor.point.y.abs_diff(y) as i32;
                return (sensor.point.x - sensor.distance + ydiff, sensor.point.x + sensor.distance - ydiff);
            })
            .filter(|(start, end)| start <= end)
            .collect::<Vec<(i32, i32)>>();
            
        let mut x = x;
        let mut moved = true;
        while moved {
            moved = false;
            for (start, end) in ranges.iter() {
                if &x >= start && &x < end {
                    moved = true;
                    x = end + 1;
                }
            }
        }
        return x;
    }

    fn is_covered(&self, point: &Point) -> bool {
        for sensor in self.sensors.iter() {
            if sensor.distance(point) <= sensor.distance {
                return true;
            }
        }
        return false;
    }
}

struct Sensor {
    point: Point,
    distance: i32,
}

impl Sensor {
    fn from(point: Point, distance: i32) -> Sensor {
        Sensor { point: point, distance: distance }
    }

    fn distance(&self, other: &Point) -> i32 {
        self.point.distance(other)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn distance(&self, other: &Point) -> i32 {
        self.x.abs_diff(other.x) as i32 + self.y.abs_diff(other.y) as i32
    }
}

fn parse_input(input: &str) -> Map {
    let regex = regex::Regex::new(r"^Sensor at x=(\d+), y=(\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
    let mut sensors = Vec::new();
    let mut beacons = HashSet::new();
    let mut furthest_beacon = None;
    let mut lowest_x = None;
    let mut highest_x = None;
    for row in input.lines() {
        let captures = regex.captures(row).unwrap();
        let x = captures[1].parse().unwrap();
        let y = captures[2].parse().unwrap();
        let beacon_x = captures[3].parse().unwrap();
        let beacon_y = captures[4].parse().unwrap();

        let sensor = Point::from(x, y);
        let beacon = Point::from(beacon_x, beacon_y);
        let distance = sensor.distance(&beacon);

        sensors.push(Sensor::from(sensor, distance));
        beacons.insert(beacon);
        furthest_beacon = match furthest_beacon {
            None => Some(distance),
            Some(current_distance) => Some(current_distance.max(distance))
        };
        lowest_x = match lowest_x {
            None => Some(x),
            Some(current_lowest_x) => Some(current_lowest_x.min(x))
        };
        highest_x = match highest_x {
            None => Some(x),
            Some(current_highest_x) => Some(current_highest_x.max(x))
        };
    }

    return Map {
        sensors: sensors,
        beacons: beacons,
        furthest_beacon: furthest_beacon.unwrap(),
        lowest_x: lowest_x.unwrap(),
        highest_x: highest_x.unwrap(),
    };
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        include_str!("./example.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = coverage_at(input, 10);

        assert_eq!(result, 26);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "4879972");
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = get_tuning_frequency(input, 20);

        assert_eq!(result, 56000011);
    }

    #[test]
    fn part2_input() {
        let result = get_solution_part2();

        assert_eq!(result, "12525726647448");
    }
}
