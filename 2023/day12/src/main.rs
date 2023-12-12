fn main() {
    let input = get_input();
    let springs = parse_input(input);

    let result_part1 = count_configurations_springs(&springs);
    println!("Part1: {}", result_part1);
}

fn count_configurations_springs(springs: &Vec<SpringRowRecord>) -> usize {
    springs
        .iter()
        .map(|spring| count_configurations_spring(spring))
        .sum()
}

fn count_configurations_spring(spring: &SpringRowRecord) -> usize {
    let result = search_configurations_spring(spring.springs.clone(), &spring.groups);
    return result;
}

fn search_configurations_spring(springs: SpringRow, groups: &Vec<usize>) -> usize {
    if springs.is_valid(groups) {
        return 1;
    }
    if springs.is_done() {
        return 0;
    }

    let unown_index = springs
        .springs
        .iter()
        .enumerate()
        .filter(|(_, &spring)| spring == SpringConfiguration::Unown)
        .map(|(index, _)| index)
        .take(1)
        .next()
        .unwrap();
    let mut spring_tmp = springs.clone();
    spring_tmp.springs[unown_index] = SpringConfiguration::Damaged;
    let result1 = search_configurations_spring(spring_tmp, groups);

    let mut spring_tmp = springs.clone();
    spring_tmp.springs[unown_index] = SpringConfiguration::Operational;
    let result2 = search_configurations_spring(spring_tmp, groups);

    return result1 + result2;
}

#[derive(Debug)]
struct SpringRowRecord {
    springs: SpringRow,
    groups: Vec<usize>,
}

impl std::fmt::Display for SpringRowRecord {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} {:?}", self.springs, self.groups)
    }
}

impl SpringRowRecord {
    fn from(str: &str) -> Self {
        let line_split = str.split(' ').collect::<Vec<_>>();
        assert!(line_split.len() == 2);
        let spring_row = SpringRow::from(line_split[0]);
        let groups = line_split[1]
            .split(',')
            .map(|str| str.parse().unwrap())
            .collect();
        let spring = SpringRowRecord {
            springs: spring_row,
            groups,
        };
        return spring;
    }
}

#[derive(Debug, Clone)]
struct SpringRow {
    springs: Vec<SpringConfiguration>,
}

impl std::fmt::Display for SpringRow {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: String = self
            .springs
            .iter()
            .map(|&spring| match spring {
                SpringConfiguration::Unown => '?',
                SpringConfiguration::Damaged => '#',
                SpringConfiguration::Operational => '.',
            })
            .collect();
        return write!(formatter, "{}", str);
    }
}

impl SpringRow {
    fn from(str: &str) -> Self {
        let springs = str.chars()
            .map(|c| SpringConfiguration::parse(c).unwrap())
            .collect::<Vec<_>>();
        return SpringRow { springs };
    }
}

impl SpringRow {
    fn is_done(&self) -> bool {
        self.springs
            .iter()
            .all(|&spring| spring != SpringConfiguration::Unown)
    }

    fn get_groups(&self) -> Vec<usize> {
        let mut current_groups = Vec::new();
        let mut current_group = 0;
        for &spring in self.springs.iter() {
            if spring == SpringConfiguration::Damaged {
                current_group += 1;
            } else if current_group > 0 {
                current_groups.push(current_group);
                current_group = 0;
            }
        }
        if current_group > 0 {
            current_groups.push(current_group);
        }
        return current_groups;
    }

    fn is_valid(&self, groups: &Vec<usize>) -> bool {
        let current_groups = self.get_groups();
        let mut all_valid = true;
        for index in 0..current_groups.len().min(groups.len()) {
            let have = current_groups[index];
            let expected = groups[index];
            if have != expected {
                all_valid = false;
            }
        }
        return all_valid && current_groups.len() == groups.len();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringConfiguration {
    Unown,
    Operational,
    Damaged,
}

impl SpringConfiguration {
    fn parse(char: char) -> Option<Self> {
        match char {
            '?' => Some(Self::Unown),
            '.' => Some(Self::Operational),
            '#' => Some(Self::Damaged),
            _ => None,
        }
    }
}

fn parse_input(input: &str) -> Vec<SpringRowRecord> {
    input.lines().map(|line| SpringRowRecord::from(line)).collect()
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("?#.", false; "with_unown")]
    #[test_case("##.", true; "without_unown")]
    fn spring_row_is_done(strings: &str, result: bool) {
        let spring_row = SpringRow::from(strings);
        assert_eq!(spring_row.is_done(), result);
    }

    #[test]
    fn spring_row_get_groups() {
        let spring_row = SpringRow {
            springs: vec![
                SpringConfiguration::Unown,
                SpringConfiguration::Damaged,
                SpringConfiguration::Unown,
                SpringConfiguration::Damaged,
                SpringConfiguration::Damaged,
                SpringConfiguration::Operational,
                SpringConfiguration::Unown,
            ],
        };

        assert_eq!(spring_row.get_groups(), vec![1, 2]);
    }

    #[test_case("?#?##.? 1,1", false; "test_example_1")]
    #[test_case("?#?##.? 1,2", true; "test_example_2")]
    #[test_case("?#?##.? 1,3", false; "test_example_3")]
    #[test_case("#...??#??##..?.#?? 1,7,1,1", false; "test_example_4")]
    fn spring_row_is_valid(str: &str, result: bool) {
        let spring = SpringRowRecord::from(str);
        assert_eq!(spring.springs.is_valid(&spring.groups), result);
    }

    fn get_example_input() -> &'static str {
        include_str!("./example.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let springs = parse_input(input);
        let result = count_configurations_springs(&springs);
        assert_eq!(result, 21);
    }

    #[test_case("????.????#?# 1,1,7", 3)]
    #[test_case("?...??#??##..?.#?? 1,7,1,1", 2)]
    fn part1_test(str: &str, expected: usize) {
        let springs = parse_input(str);
        let result = count_configurations_springs(&springs);
        assert_eq!(result, expected);
    }
}
