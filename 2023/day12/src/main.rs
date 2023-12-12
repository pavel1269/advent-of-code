use std::collections::HashMap;

fn main() {
    let input = get_input();
    let mut cache = Cache::new();

    let springs = parse_input(input, 1);
    let result_part1 = count_configurations_springs(&springs, Some(&mut cache));
    println!("Part1: {} (cache hits/entries: {}/{})", result_part1, cache.hits, cache.cache.len());

    cache.hits = 0;
    let springs = parse_input(input, 5);
    let result_part2 = count_configurations_springs(&springs, Some(&mut cache));
    println!("Part2: {} (cache hits/entries: {}/{})", result_part2, cache.hits, cache.cache.len());
}

fn count_configurations_springs(
    springs: &Vec<SpringRowRecord>,
    mut cache: Option<&mut Cache>,
) -> usize {
    springs
        .iter()
        .map(|spring| count_configurations_spring(spring, &mut cache))
        .sum()
}

fn count_configurations_spring(spring: &SpringRowRecord, cache: &mut Option<&mut Cache>) -> usize {
    let result =
        search_configurations_spring(spring.springs.clone(), spring.groups.clone(), 0, cache);
    return result;
}

fn search_configurations_spring(
    springs: SpringRow,
    groups: Vec<usize>,
    current_size: usize,
    cache: &mut Option<&mut Cache>,
) -> usize {
    match springs.springs.first() {
        None => {
            if groups.len() == 0 && current_size == 0 {
                return 1;
            } else if groups.len() == 1 && current_size > 0 {
                if let Some(&count) = groups.first() {
                    if count == current_size {
                        return 1;
                    }
                }
            }
            return 0;
        }
        Some(SpringConfiguration::Operational) => {
            if current_size == 0 {
                return search_configurations_spring(springs.clone_next(true), groups, 0, cache);
            } else {
                if let Some(&count) = groups.first() {
                    if count == current_size {
                        return search_configurations_spring(
                            springs.clone_next(true),
                            groups.iter().skip(1).copied().collect(),
                            0,
                            cache,
                        );
                    }
                }
                return 0;
            }
        }
        Some(SpringConfiguration::Damaged) => {
            if let Some(&count) = groups.first() {
                if current_size < count {
                    return search_configurations_spring(
                        springs.clone_next(false),
                        groups,
                        current_size + 1,
                        cache,
                    );
                }
            }
            return 0;
        }
        Some(SpringConfiguration::Unown) => {
            if let Some(cache) = cache {
                if let Some(result) = cache.get(&springs, &groups, current_size) {
                    return result;
                }
            }
            let result1 = search_configurations_spring(
                springs.clone_replace_first(SpringConfiguration::Damaged),
                groups.clone(),
                current_size,
                cache,
            );
            let result2 = search_configurations_spring(
                springs.clone_replace_first(SpringConfiguration::Operational),
                groups.clone(),
                current_size,
                cache,
            );
            let result = result1 + result2;
            if let Some(cache) = cache {
                cache.insert(&springs, &groups, current_size, result);
            }
            return result;
        }
    };
}

struct Cache {
    cache: HashMap<(SpringRow, Vec<usize>, usize), usize>,
    hits: usize,
}

impl Cache {
    fn new() -> Self {
        Cache {
            cache: HashMap::new(),
            hits: 0,
        }
    }

    fn insert(&mut self, springs: &SpringRow, groups: &Vec<usize>, current_size: usize, value: usize) {
        self.cache.insert((springs.clone(), groups.clone(), current_size), value);
    }

    fn get(&mut self, springs: &SpringRow, groups: &Vec<usize>, current_size: usize) -> Option<usize> {
        let result = self.cache.get(&(springs.clone(), groups.clone(), current_size)).copied();
        if let Some(_) = result {
            self.hits += 1;
        }
        return result;
    }
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
    fn from(str: &str, times: usize) -> Self {
        let line_split = str.split(' ').collect::<Vec<_>>();
        assert!(line_split.len() == 2);
        let spring_row = SpringRow::from(line_split[0], times);
        let groups = Self::parse_groups(line_split[1], times);
        let spring = SpringRowRecord {
            springs: spring_row,
            groups,
        };
        return spring;
    }

    fn parse_groups(str: &str, times: usize) -> Vec<usize> {
        let groups = str
            .split(',')
            .map(|str| str.parse().unwrap())
            .collect::<Vec<usize>>();
        let mut groups_final = Vec::with_capacity(groups.len() * times);
        for _ in 0..times {
            groups_final.append(&mut groups.clone());
        }
        return groups_final;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    fn clone_next(&self, skip: bool) -> Self {
        let springs = self
            .springs
            .iter()
            .skip(1)
            .skip_while(|&&c| skip && c == SpringConfiguration::Operational)
            .copied()
            .collect();
        return SpringRow { springs };
    }

    fn clone_replace_first(&self, configuration: SpringConfiguration) -> Self {
        let mut result = self.clone();
        result.springs[0] = configuration;
        return result;
    }

    fn from(str: &str, times: usize) -> Self {
        let springs = str
            .chars()
            .map(|c| SpringConfiguration::parse(c).unwrap())
            .collect::<Vec<_>>();
        let mut springs_final = Vec::with_capacity(springs.len() * times + times - 1);
        for time in 1..=times {
            springs_final.append(&mut springs.clone());
            if time != times {
                springs_final.push(SpringConfiguration::Unown);
            }
        }
        return SpringRow {
            springs: springs_final,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn parse_input(input: &str, times: usize) -> Vec<SpringRowRecord> {
    input
        .lines()
        .map(|line| SpringRowRecord::from(line, times))
        .collect()
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn get_example_input() -> &'static str {
        include_str!("./example.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let springs = parse_input(input, 1);
        let result = count_configurations_springs(&springs, None);
        assert_eq!(result, 21);
    }

    #[test_case("????.????#?# 1,1,7", 3; "test_example_1")]
    #[test_case("?...??#??##..?.#?? 1,7,1,1", 2; "test_example_2")]
    fn part1_test(str: &str, expected: usize) {
        let springs = parse_input(str, 1);
        let result = count_configurations_springs(&springs, None);
        assert_eq!(result, expected);
    }

    #[test_case(false)]
    #[test_case(true)]
    fn part2_example(use_cache: bool) {
        let mut cache_tmp = Cache::new();
        let mut cache = None;
        if use_cache {
            cache = Some(&mut cache_tmp);
        }
        let input = get_example_input();
        let springs = parse_input(input, 5);
        let result = count_configurations_springs(&springs, cache);
        assert_eq!(result, 525152);
    }

    #[test_case("???.### 1,1,3", 1; "test_example_1")]
    #[test_case(".??..??...?##. 1,1,3", 16384; "test_example_2")]
    fn part2_test(str: &str, expected: usize) {
        let springs = parse_input(str, 5);
        let result = count_configurations_springs(&springs, None);
        assert_eq!(result, expected);
    }
}
