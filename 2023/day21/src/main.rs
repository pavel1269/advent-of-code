use std::collections::HashSet;

fn main() {
    let input = get_input();
    let result_part1 = part1(&input);
    println!("Part1: {}", result_part1);
}

fn part1(input: &str) -> usize {
    let (map, start) = Map::from(input);
    let result = map.reachable_after_steps(&start, 64);
    return result;
}

#[derive(Debug)]
struct Map {
    width: isize,
    height: isize,
    gardens: HashSet<Position>,
}

impl Map {
    fn reachable_after_steps(&self, pos: &Position, steps: usize) -> usize {
        let mut positions_now = HashSet::from([pos.clone()]);
        for _ in 0..steps {
            let positions_new = positions_now
                .into_iter()
                .flat_map(|pos| pos.step())
                .filter(|pos| self.is_valid(pos))
                .filter(|pos| self.gardens.contains(pos))
                .collect();

            positions_now = positions_new;
        }
        return positions_now.len();
    }

    fn is_valid(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width && pos.y < self.height
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
        let height = tiles.iter().map(|(pos, _)| pos.y).max().unwrap();
        let width = tiles.iter().map(|(pos, _)| pos.x).max().unwrap();
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
        let result = map.reachable_after_steps(&start, steps);
        assert_eq!(expect, result);
    }
}
