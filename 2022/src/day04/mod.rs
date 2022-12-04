
pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = calculate_total_overlaps(input);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = calculate_overlaps(input);
    return result.to_string();
}

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn is_complete_overlap(&self, other: &Range) -> bool {
        if self.start >= other.start && self.end <= other.end {
            return true;
        }
        return self.start <= other.start && self.end >= other.end;
    }
    
    fn is_overlap(&self, other: &Range) -> bool {
        if self.is_complete_overlap(other) {
            return true;
        }
        if self.start >= other.start && self.start <= other.end {
            return true;
        }
        return self.end >= other.start && self.end <= other.end;
    }
}

fn calculate_overlaps(input: &str) -> u32 {
    let pairs = parse_input(input);
    let mut count = 0;
    pairs.iter().for_each(|(range1, range2)| {
        if range1.is_overlap(range2) {
            count += 1;
        }
    });
    return count;
}

fn calculate_total_overlaps(input: &str) -> u32 {
    let pairs = parse_input(input);
    let mut count = 0;
    pairs.iter().for_each(|(range1, range2)| {
        if range1.is_complete_overlap(range2) {
            count += 1;
        }
    });
    return count;
}

fn parse_input(input: &str) -> Vec::<(Range, Range)> {
    let input_lines = input.lines();
    let mut pairs= Vec::with_capacity(input_lines.clone().count());
    let regex = regex::Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    input_lines.for_each(|line| {
        match regex.captures(line) {
            None => panic!("Could not parse {}", line),
            Some(captures) => {
                let range1 = Range {
                    start: captures[1].parse().unwrap(),
                    end: captures[2].parse().unwrap(),
                };
                let range2 = Range {
                    start: captures[3].parse().unwrap(),
                    end: captures[4].parse().unwrap(),
                };
                pairs.push((range1, range2));
            },
        }
    });

    return pairs;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
    }
    
    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = calculate_total_overlaps(input);

        assert_eq!(result, 2);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "530");
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = calculate_overlaps(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn part2_input() {
        let result = get_solution_part2();

        assert_eq!(result, "903");
    }
}
