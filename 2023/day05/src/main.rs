use std::{collections::HashMap, str::Lines};

fn main() {
    let input = get_input();
    let alamanac = parse_input(input);

    let result_part1 = get_lowest_location(&alamanac);
    println!("Part1: {}", result_part1);

    let result_part2 = get_lowest_location_range(&alamanac);
    println!("Part2: {}", result_part2);
}

#[derive(Debug, Clone, Copy)]
struct Range {
    from: i64,
    to: i64,
}

fn get_lowest_location_range(almanac: &Almanac) -> i64 {
    let mut ranges = generate_ranges(&almanac.seeds);
    let mut at = "seed".to_string();
    while almanac.map_name.contains_key(&at) {
        let mappings = &almanac.map_mappings[&at];
        ranges = ranges
            .iter()
            .map(|range| mappings.apply_range(range))
            .flatten()
            .collect();
        at = almanac.map_name[&at].clone();
    }

    return ranges.iter().map(|range| range.from).min().unwrap();
}

fn generate_ranges(seeds: &Vec<i64>) -> Vec<Range> {
    if seeds.len() % 2 != 0 {
        panic!()
    }

    let mut ranges = Vec::new();
    for index in 0..seeds.len() {
        if index % 2 == 1 {
            continue;
        }

        let from = seeds[index];
        let range = seeds[index + 1];
        let to = from + range - 1;
        ranges.push(Range { from, to });
    }

    return ranges;
}

fn get_lowest_location(almanac: &Almanac) -> i64 {
    let mut numbers = almanac.seeds.clone();
    let mut at = "seed".to_string();
    while almanac.map_name.contains_key(&at) {
        let mappings = &almanac.map_mappings[&at];
        numbers = numbers
            .iter()
            .map(|number| mappings.apply(*number))
            .collect();
        at = almanac.map_name[&at].clone();
    }

    return *numbers.iter().min().unwrap();
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    map_name: HashMap<String, String>,
    map_mappings: HashMap<String, Mappings>,
}

impl Almanac {
    fn new(seeds: Vec<i64>) -> Self {
        return Almanac {
            seeds,
            map_name: HashMap::new(),
            map_mappings: HashMap::new(),
        };
    }

    fn add(&mut self, from: String, to: String, mappings: Mappings) {
        self.map_mappings.insert(from.clone(), mappings);
        self.map_name.insert(from, to);
    }
}

#[derive(Debug)]
struct Mappings {
    mappings: Vec<Mapping>,
}

impl Mappings {
    fn apply(&self, number: i64) -> i64 {
        for mapping in self.mappings.iter() {
            match mapping.try_apply(number) {
                None => continue,
                Some(number_new) => return number_new,
            }
        }
        return number;
    }

    fn apply_range(&self, range: &Range) -> Vec<Range> {
        let mut start_numbers: Vec<i64> = self.mappings.iter().map(|rule| rule.start).filter(|number| *number > range.from && *number < range.to).collect();
        start_numbers.sort();
        let mut ranges = Vec::new();
        let mut at = range.from;
        while at < range.to {
            let next = match start_numbers.pop() {
                None => range.to + 1,
                Some(next) => next,
            };
            let offset = self.mappings.iter().filter(|rule| rule.start <= at && rule.end >= at).map(|rule| rule.offset).next();
            match offset {
                None => {
                    ranges.push(Range { from: at, to: next - 1});
                },
                Some(offset) => {
                    ranges.push(Range { from: at - offset, to: next - offset - 1});
                },
            }

            at = next;
        }

        return ranges;
    }
}

#[derive(Debug)]
struct Mapping {
    start: i64,
    end: i64,
    offset: i64,
}

impl Mapping {
    fn from(numbers: Vec<i64>) -> Self {
        if numbers.len() != 3 {
            panic!()
        }

        let start_to = numbers[0];
        let start_from = numbers[1];
        let range = numbers[2];

        let mapping = Mapping {
            start: start_from,
            end: start_from + range - 1,
            offset: start_from - start_to,
        };
        return mapping;
    }

    fn try_apply(&self, number: i64) -> Option<i64> {
        if number < self.start || number > self.end {
            return None;
        }

        return Some(number - self.offset);
    }
}

fn parse_input(input: &str) -> Almanac {
    let mut lines = input.lines();
    let seeds_string = lines.next().unwrap();
    let seeds: Vec<i64> = seeds_string[7..]
        .split(' ')
        .map(|seed| seed.parse().unwrap())
        .collect();
    lines.next();

    let regex = regex::Regex::new(r"^(.+)-to-(.+) map:$").unwrap();
    let mut almanac = Almanac::new(seeds);
    while let Some(line) = lines.next() {
        let captures = regex.captures(line).unwrap();
        let name_from = captures[1].to_string();
        let name_to = captures[2].to_string();
        let mappings = parse_ranges(&mut lines);
        almanac.add(name_from, name_to, mappings);
    }
    return almanac;
}

fn parse_ranges(lines: &mut Lines<'_>) -> Mappings {
    let mut mappings = Vec::new();
    while let Some(line) = lines.next() {
        if line.len() == 0 {
            break;
        }

        let numbers: Vec<i64> = line
            .split(' ')
            .map(|number| number.parse().unwrap())
            .collect();
        mappings.push(Mapping::from(numbers));
    }

    return Mappings { mappings };
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
        let alamanac = parse_input(input);
        let result = get_lowest_location(&alamanac);
        assert_eq!(result, 35);
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let alamanac = parse_input(input);
        let result = get_lowest_location_range(&alamanac);
        assert_eq!(result, 46);
    }
}
