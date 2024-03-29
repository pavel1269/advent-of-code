use std::collections::HashSet;
use criterion::{Criterion, black_box};

fn main() {
    let input = get_input();
    let map = parse_input(input);

    let result_part1 = sum_distances(&map, 2);
    println!("Part1: {}", result_part1);

    let result_part2 = sum_distances(&map, 1000000);
    println!("Part2: {}", result_part2);
    
    let mut criterion = Criterion::default();
    criterion.bench_function("2023 day 11 part 1", |b| b.iter(|| { black_box(sum_distances(&map, 2)); }));
    criterion.bench_function("2023 day 11 part 2", |b| b.iter(|| { black_box(sum_distances(&map, 1000000)); }));
}

fn sum_distances(map: &Map, gap_size: usize) -> usize {
    let mut sum = 0;
    let galaxies = map.galaxies.len();
    for (index, galaxy) in map.galaxies.iter().enumerate() {
        for index_other in index+1..galaxies {
            let galaxy_other = map.galaxies[index_other];
            let distance = map.calculate_distance(galaxy, &galaxy_other, gap_size);
            sum += distance;
        }
    }
    return sum;
}

#[derive(Debug)]
struct Map {
    galaxies: Vec<Point>,
    column_gaps: HashSet<usize>,
    row_gaps: HashSet<usize>,
}

impl Map {
    fn calculate_distance(&self, point1: &Point, point2: &Point, gap_size: usize) -> usize {
        let x_from = point1.x.min(point2.x);
        let x_to = point1.x.max(point2.x);
        let y_from = point1.y.min(point2.y);
        let y_to = point1.y.max(point2.y);

        let column_gaps = (x_from..x_to).filter(|x| self.column_gaps.contains(x)).count();
        let row_gaps = (y_from..y_to).filter(|y| self.row_gaps.contains(y)).count();
        let gap_size_calc = gap_size - 1;
        let distance = x_to - x_from + column_gaps * gap_size_calc + y_to - y_from + row_gaps * gap_size_calc;
        return distance;
    }

    fn from(galaxies: &Vec<Point>) -> Self {
        let galaxies = galaxies.clone();

        let columns = galaxies.iter().map(|point| point.x).collect::<HashSet<_>>();
        let width = galaxies.iter().map(|point| point.x).max().unwrap();
        let column_gaps = (0..width).filter(|x| !columns.contains(x)).collect::<HashSet<_>>();

        let rows = galaxies.iter().map(|point| point.y).collect::<HashSet<_>>();
        let height = galaxies.iter().map(|point| point.y).max().unwrap();
        let row_gaps = (0..height).filter(|y| !rows.contains(y)).collect::<HashSet<_>>();

        let map = Map {
            galaxies,
            column_gaps,
            row_gaps,
        };
        return map;
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> Map {
    let mut galaxies = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push(Point { x, y });
            }
        }
    }
    let map = Map::from(&galaxies);
    return map;
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
        let map = parse_input(input);
        let result = sum_distances(&map, 2);
        assert_eq!(result, 374);
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let map = parse_input(input);
        let result = sum_distances(&map, 10);
        assert_eq!(result, 1030);
    }
}
