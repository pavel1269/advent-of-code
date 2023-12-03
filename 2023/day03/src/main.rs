fn main() {
    let input = get_input();
    let schematics = parse_input(input, false);
    let result_part1 = sum_touching_numbers(&schematics);
    println!("Part1: {}", result_part1);

    let schematics_gears = parse_input(input, true);
    let result_part2 = sum_gears(&schematics_gears);
    println!("Part2: {}", result_part2);
}

fn sum_gears(schematics: &ParsedSchemtics) -> u64 {
    let mut sum = 0;
    for gear in schematics.symbols.iter() {
        let mut parts = Vec::new();
        for part in schematics.part_numbers.iter() {
            for point in part.points.iter() {
                if point.is_nearby(gear) {
                    parts.push(part.id);
                    break;
                }
            }
        }

        if parts.len() < 2 {
            continue;
        }
        else if parts.len() > 2 {
            panic!();
        }
        else {
            sum += (parts[0] as u64) * (parts[1] as u64);
        }
    }
    return sum;
}

fn sum_touching_numbers(schematics: &ParsedSchemtics) -> u64 {
    let mut sum = 0;
    for part in schematics.part_numbers.iter() {
        let mut decided = false;
        for point in part.points.iter() {
            for symbol_point in schematics.symbols.iter() {
                if point.is_nearby(symbol_point) {
                    sum += part.id as u64;
                    decided = true;
                    break;
                }
            }
            if decided {
                break;
            }
        }
    }
    return sum;
}

#[derive(Debug)]
struct ParsedSchemtics {
    part_numbers: Vec<ParsedPartNumber>,
    symbols: Vec<Point>,
}

#[derive(Debug)]
struct ParsedPartNumber {
    id: u32,
    points: Vec<Point>,
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn is_nearby(&self, other: &Self) -> bool {
        let x_diff = self.x.abs_diff(other.x);
        let y_diff = self.y.abs_diff(other.y);
        return x_diff <= 1 && y_diff <= 1;
    }
}

fn parse_input(input: &str, only_gears: bool) -> ParsedSchemtics {
    let mut symbols = Vec::new();
    let mut part_numbers = Vec::new();

    for (x, line) in input.lines().enumerate() {
        let mut loading_number = false;
        let mut number = 0;
        let mut number_points = Vec::new();
        for (y, char) in line.chars().enumerate() {
            if char == '.' {
                if loading_number {
                    part_numbers.push(ParsedPartNumber { id: number, points: number_points });

                    loading_number = false;
                    number = 0;
                    number_points = Vec::new();
                }
                continue;
            }
            else if char >= '0' && char <= '9' {
                loading_number = true;
                number *= 10;
                number += char.to_digit(10).unwrap();
                number_points.push(Point { x, y });
            }
            else {
                if loading_number {
                    part_numbers.push(ParsedPartNumber { id: number, points: number_points });

                    loading_number = false;
                    number = 0;
                    number_points = Vec::new();
                }

                if only_gears {
                    if char == '*' {
                        symbols.push(Point { x, y });
                    }
                }
                else {
                    symbols.push(Point { x, y });
                }
            }
        }

        if loading_number {
            part_numbers.push(ParsedPartNumber { id: number, points: number_points });
        }
    }

    return ParsedSchemtics {
        symbols: symbols,
        part_numbers: part_numbers,
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
        let schematics = parse_input(input, false);
        let result = sum_touching_numbers(&schematics);

        assert_eq!(result, 4361);
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let schematics = parse_input(input, true);
        let result = sum_gears(&schematics);

        assert_eq!(result, 467835);
    }
}
