use std::collections::HashSet;


pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = coverage_at(input, 2000000);
    return result.to_string();
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
}
