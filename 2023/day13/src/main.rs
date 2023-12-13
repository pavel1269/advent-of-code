fn main() {
    let input = get_input();
    let maps = parse_input(input);
    let result_part1 = summarize_mirrors(&maps, false);
    println!("Part1: {}", result_part1);
    let result_part2 = summarize_mirrors(&maps, true);
    println!("Part2: {}", result_part2);
}

fn summarize_mirrors(maps: &Vec<Map>, include_error: bool) -> u64 {
    maps.iter()
        .map(|map| map.detect_mirror_sumary(include_error))
        .sum()
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    rows: Vec<u64>,
    columns: Vec<u64>,
}

impl Map {
    fn detect_mirror_sumary(&self, include_error: bool) -> u64 {
        for x in 0..self.width - 1 {
            if self.is_reflected_row(x, include_error) {
                return x as u64 + 1;
            }
        }
        for y in 0..self.height - 1 {
            if self.is_reflected_column(y, include_error) {
                return (y as u64 + 1) * 100;
            }
        }
        panic!();
    }

    fn is_reflected_column(&self, y: usize, include_error: bool) -> bool {
        Self::is_reflected(&self.rows, y, self.height, include_error)
    }

    fn is_reflected_row(&self, x: usize, include_error: bool) -> bool {
        Self::is_reflected(&self.columns, x, self.width, include_error)
    }

    fn is_reflected(vec: &Vec<u64>, mut index: usize, limit: usize, include_error: bool) -> bool {
        let mut index2 = index + 1;
        let mut errors = 0;
        while index2 < limit {
            if Self::is_single_error(vec[index], vec[index2]) {
                errors += 1;
                if errors > 1 {
                    return false;
                }
            } else if vec[index] != vec[index2] {
                return false;
            }

            if let Some(index_new) = index.checked_sub(1) {
                index = index_new;
            } else {
                break;
            }
            index2 += 1;
        }

        if include_error {
            return errors == 1;
        } else {
            return errors == 0;
        }
    }

    fn is_single_error(n1: u64, n2: u64) -> bool {
        let x = n1 ^ n2;
        if x == 0 {
            return false;
        }
        return x & (x - 1) == 0;
    }

    fn parse(input: &Vec<&str>) -> Self {
        let height = input.len();
        let width = input[0].len();
        let mut rows = vec![0; height];
        let mut columns = vec![0; width];

        for (y, line) in input.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    rows[y] |= 1 << x;
                    columns[x] |= 1 << y;
                }
            }
        }

        let map = Map {
            width,
            height,
            rows,
            columns,
        };
        return map;
    }
}

fn parse_input(input: &str) -> Vec<Map> {
    let mut maps = Vec::new();
    let mut current_input_map = Vec::new();
    for line in input.lines() {
        if line.len() > 0 {
            current_input_map.push(line);
        } else {
            let map = Map::parse(&current_input_map);
            maps.push(map);
            current_input_map = Vec::new();
        }
    }
    if current_input_map.len() > 0 {
        let map = Map::parse(&current_input_map);
        maps.push(map);
    }
    return maps;
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
        let maps = parse_input(input);
        let result = summarize_mirrors(&maps, false);
        assert_eq!(result, 405);
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let maps = parse_input(input);
        let result = summarize_mirrors(&maps, true);
        assert_eq!(result, 400);
    }
}
