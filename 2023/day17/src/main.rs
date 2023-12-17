use std::collections::HashMap;

fn main() {
    let input = get_input();
    let result_part1 = part1(input);
    println!("Part1: {}", result_part1);
    let result_part2 = part2(input);
    println!("Part2: {}", result_part2);
}

fn part1(input: &str) -> u32 {
    let map = Map::from(input);
    let result = map.traverse(
        &Position { x: 0, y: 0 },
        &Position {
            x: map.grid[0].len() - 1,
            y: map.grid.len() - 1,
        },
        0,
        3,
    );
    return result;
}

fn part2(input: &str) -> u32 {
    let map = Map::from(input);
    let result = map.traverse(
        &Position { x: 0, y: 0 },
        &Position {
            x: map.grid[0].len() - 1,
            y: map.grid.len() - 1,
        },
        4,
        10,
    );
    return result;
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<u32>>,
}

impl Map {
    fn traverse(&self, from: &Position, to: &Position, min: u8, max: u8) -> u32 {
        let start = Path {
            cost: 0,
            position: from.clone(),
            last_direction: Direction::Up,
            last_direction_times: min,
        };
        let mut paths = vec![start];
        let mut cache = Cache::new();
        let mut final_cost = u32::MAX;
        while let Some(path) = paths.pop() {
            if path.cost > final_cost {
                continue;
            }
            if cache.check_cache(&path) {
                continue;
            }
            if &path.position == to && path.last_direction_times >= min {
                final_cost = final_cost.min(path.cost);
            }
            let mut next_moves = path.next_moves(self, min, max);
            paths.append(&mut next_moves);
        }

        return final_cost;
    }

    fn from(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();
        let mut grid = vec![vec![0; width]; height];
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let loss = char as u32 - '0' as u32;
                grid[y][x] = loss;
            }
        }
        let map = Map { grid };
        return map;
    }
}

struct Cache {
    cached: HashMap<(Position, Direction, u8), u32>,
}

impl Cache {
    fn check_cache(&mut self, path: &Path) -> bool {
        if let Some(&cost) = self.cached.get(&(
            path.position,
            path.last_direction,
            path.last_direction_times,
        )) {
            if cost <= path.cost {
                return true;
            }
        }

        self.cached
            .entry((path.position, path.last_direction, path.last_direction_times))
            .and_modify(|cost| *cost = path.cost.min(*cost))
            .or_insert(path.cost);
        return false;
    }

    fn new() -> Self {
        Cache {
            cached: HashMap::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Path {
    cost: u32,
    position: Position,
    last_direction: Direction,
    last_direction_times: u8,
}

impl Path {
    fn next_moves(&self, map: &Map, min: u8, max: u8) -> Vec<Self> {
        let mut moves = Vec::new();
        if self.last_direction_times < min {
            if let Some(path) = self.new_move(map, self.last_direction, max) {
                moves.push(path);
            }
        } else {
            if let Some(path) = self.new_move(map, Direction::Left, max) {
                moves.push(path);
            }
            if let Some(path) = self.new_move(map, Direction::Right, max) {
                moves.push(path);
            }
            if let Some(path) = self.new_move(map, Direction::Up, max) {
                moves.push(path);
            }
            if let Some(path) = self.new_move(map, Direction::Down, max) {
                moves.push(path);
            }
        }
        return moves;
    }

    fn new_move(&self, map: &Map, direction: Direction, max: u8) -> Option<Self> {
        if direction.is_opposite(self.last_direction) {
            return None;
        }

        let last_direction_times = if self.last_direction == direction {
            self.last_direction_times + 1
        } else {
            1
        };
        if last_direction_times > max {
            return None;
        }

        let position = self.position.step(map, direction);
        if position.is_none() {
            return None;
        };

        let position = position.unwrap();
        let cost = map.grid[position.y][position.x] + self.cost;
        let path = Path {
            cost,
            position,
            last_direction: direction,
            last_direction_times,
        };
        return Some(path);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn step(&self, map: &Map, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Up => {
                if let Some(y) = self.y.checked_add_signed(-1) {
                    return Some(Position { x: self.x, y });
                }
                return None;
            }
            Direction::Down => {
                if self.y < map.grid.len() - 1 {
                    return Some(Position {
                        x: self.x,
                        y: self.y + 1,
                    });
                }
                return None;
            }
            Direction::Left => {
                if let Some(x) = self.x.checked_add_signed(-1) {
                    return Some(Position { x, y: self.y });
                }
                return None;
            }
            Direction::Right => {
                if self.x < map.grid[0].len() - 1 {
                    return Some(Position {
                        x: self.x + 1,
                        y: self.y,
                    });
                }
                return None;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_opposite(&self, other: Direction) -> bool {
        match self {
            Direction::Up => return other == Direction::Down,
            Direction::Down => return other == Direction::Up,
            Direction::Left => return other == Direction::Right,
            Direction::Right => return other == Direction::Left,
        }
    }
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
        let result = part1(input);
        assert_eq!(result, 102);
    }

    fn get_example2_input() -> &'static str {
        include_str!("./example2.txt")
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = part2(input);
        assert_eq!(result, 94);
    }
    #[test]
    fn part2_example2() {
        let input = get_example2_input();
        let result = part2(input);
        assert_eq!(result, 71);
    }
}
