use std::collections::HashSet;

fn main() {
    let input = get_input();
    let result_part1 = part1(&input);
    println!("Part1: {}", result_part1);
    let result_part2 = part2(&input);
    println!("Part2: {}", result_part2);
}

fn part1(input: &str) -> usize {
    let (map, start) = Map::from(input);
    let result = map.reachable_after_steps(&start, 64, false);
    return result;
}

fn part2(input: &str) -> usize {
    let steps = 26501365;
    let cycles = (steps - 65) / 131;
    assert!(cycles * 131 + 65 == steps);

    let (map, start) = Map::from(input);
    let result_65 = map.reachable_after_steps(&start, 65, true);
    let result_196 = map.reachable_after_steps(&start, 196, true);
    let result_327 = map.reachable_after_steps(&start, 327, true);

    let d1_1_cycle = result_196 - result_65;
    let d1_2_cycle = result_327 - result_196;
    let d2_cycle = d1_2_cycle - d1_1_cycle;

    // (ddx) a = 30270
    // (dx) 30270 x + b = 0
    // (dx for x = 0) b = 116
    // (dx for x = 1) 30270 + 116 = 30386
    // (dx for x = 2) 2 * 30270 + 116 = 60656
    // (dx for x = 3) 3 * 30270 + 116 = 90926
    // (dx) 30270 x + 116 = 0
    // 15135 x^2 + 116 x + c = 0 // ?? doesn't work for any x so sum dx over cycles

    let b = d1_1_cycle - d2_cycle;
    let a = d1_1_cycle - b;

    let mut result = result_65;
    for x in 1..=cycles {
        let result_dx = a * x + b;
        result += result_dx;
    }
    return result;
}

#[derive(Debug)]
struct Map {
    width: isize,
    height: isize,
    gardens: HashSet<Position>,
}

impl Map {
    fn reachable_after_steps(&self, pos: &Position, steps: usize, infinite: bool) -> usize {
        let mut positions_now = HashSet::from([pos.clone()]);
        for _step in 1..=steps {
            let positions_new = positions_now
                .into_iter()
                .flat_map(|pos| pos.step())
                .filter(|pos| infinite || self.is_pos_valid_within(pos))
                .filter(|pos| self.gardens.contains(&pos.wrap_by(self)))
                .collect();

            positions_now = positions_new;
        }
        return positions_now.len();
    }

    fn is_pos_valid_within(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width && pos.y < self.height
    }

    #[allow(dead_code)]
    fn print(&self, positions: &HashSet<Position>) {
        let min_x = positions.iter().map(|pos| pos.x).min().unwrap() - 2;
        let max_x = positions.iter().map(|pos| pos.x).max().unwrap() + 2;
        let min_y = positions.iter().map(|pos| pos.y).min().unwrap() - 2;
        let max_y = positions.iter().map(|pos| pos.y).max().unwrap() + 2;

        for y in min_y..=max_y {
            let line: String = (min_x..=max_x)
                .map(move |x| {
                    let pos = Position { x, y };
                    if positions.contains(&pos) {
                        'O'
                    } else if self.gardens.contains(&pos.wrap_by(self)) {
                        '.'
                    } else {
                        '#'
                    }
                })
                .collect();
            println!("{}", line);
        }
    }

    fn from(str: &str) -> (Self, Position) {
        let tiles: Vec<_> = str
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .map(|char| Tile::from(char))
                    .enumerate()
                    .map(move |(x, tile)| {
                        (
                            Position {
                                x: x as isize,
                                y: y as isize,
                            },
                            tile.unwrap(),
                        )
                    })
            })
            .collect();
        let start = tiles
            .iter()
            .filter(|(_, tile)| tile == &Tile::Start)
            .next()
            .unwrap()
            .0;
        let height = tiles.iter().map(|(pos, _)| pos.y).max().unwrap() + 1;
        let width = tiles.iter().map(|(pos, _)| pos.x).max().unwrap() + 1;
        let gardens: HashSet<_> = tiles
            .into_iter()
            .filter(|(_, tile)| tile != &Tile::Rock)
            .map(|(pos, _)| pos)
            .collect();
        let result = Self {
            width,
            height,
            gardens,
        };
        return (result, start);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn wrap_by(&self, map: &Map) -> Self {
        let x = Self::wrap(self.x, map.width);
        let y = Self::wrap(self.y, map.height);
        let result = Self { x, y };
        return result;
    }

    fn wrap(mut coord: isize, max: isize) -> isize {
        while coord < 0 {
            coord += max;
        }
        coord = coord % max;
        return coord;
    }

    fn step(&self) -> Vec<Self> {
        vec![
            Position {
                x: self.x + 1,
                y: self.y,
            },
            Position {
                x: self.x - 1,
                y: self.y,
            },
            Position {
                x: self.x,
                y: self.y + 1,
            },
            Position {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Start,
    Garden,
    Rock,
}

impl Tile {
    fn from(char: char) -> Option<Self> {
        match char {
            'S' => Some(Tile::Start),
            '.' => Some(Tile::Garden),
            '#' => Some(Tile::Rock),
            _ => None,
        }
    }
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

    #[test_case(1, 2)]
    #[test_case(6, 16)]
    fn part1_example(steps: usize, expect: usize) {
        let input = get_example_input();
        let (map, start) = Map::from(input);
        let result = map.reachable_after_steps(&start, steps, false);
        assert_eq!(expect, result);
    }

    #[test_case(6, 16)]
    #[test_case(10, 50)]
    #[test_case(50, 1594)]
    #[test_case(100, 6536)]
    fn part2_example(steps: usize, expect: usize) {
        let input = get_example_input();
        let (map, start) = Map::from(input);
        let result = map.reachable_after_steps(&start, steps, true);
        assert_eq!(expect, result);
    }
}
