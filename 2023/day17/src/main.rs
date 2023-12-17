use std::collections::HashMap;

fn main() {
    let input = get_input();
    let result_part1 = part1(input);
    println!("Part1: {}", result_part1);
}

fn part1(input: &str) -> u32 {
    let map = Map::from(input);
    let result = map.traverse(
        &Position { x: 0, y: 0 },
        &Position {
            x: map.grid[0].len() - 1,
            y: map.grid.len() - 1,
        },
    );
    return result;
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<u32>>,
}

impl Map {
    fn traverse(&self, from: &Position, to: &Position) -> u32 {
        let start = Path {
            cost: 0,
            position: from.clone(),
            last_direction: Direction::Up,
            last_direction_times: 0,
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
            if &path.position == to {
                final_cost = final_cost.min(path.cost);
            }
            let mut next_moves = path.next_moves(self);
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

        self.cached.insert(
            (
                path.position,
                path.last_direction,
                path.last_direction_times,
            ),
            path.cost,
        );
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
    fn next_moves(&self, map: &Map) -> Vec<Self> {
        let mut moves = Vec::new();
        if self.position.y > 0
            && self.last_direction != Direction::Down
            && !(self.last_direction == Direction::Up && self.last_direction_times >= 3)
        {
            let y = self.position.y - 1;
            let x = self.position.x;
            let position = Position { x, y };
            let path = self.new_move(map, position, Direction::Up);
            moves.push(path);
        }
        if self.position.y < map.grid.len() - 1
            && self.last_direction != Direction::Up
            && !(self.last_direction == Direction::Down && self.last_direction_times >= 3)
        {
            let y = self.position.y + 1;
            let x = self.position.x;
            let position = Position { x, y };
            let path = self.new_move(map, position, Direction::Down);
            moves.push(path);
        }
        if self.position.x > 0
            && self.last_direction != Direction::Right
            && !(self.last_direction == Direction::Left && self.last_direction_times >= 3)
        {
            let x = self.position.x - 1;
            let y = self.position.y;
            let position = Position { x, y };
            let path = self.new_move(map, position, Direction::Left);
            moves.push(path);
        }
        if self.position.x < map.grid[0].len() - 1
            && self.last_direction != Direction::Left
            && !(self.last_direction == Direction::Right && self.last_direction_times >= 3)
        {
            let x = self.position.x + 1;
            let y = self.position.y;
            let position = Position { x, y };
            let path = self.new_move(map, position, Direction::Right);
            moves.push(path);
        }
        return moves;
    }

    fn new_move(&self, map: &Map, position: Position, direction: Direction) -> Self {
        let cost = map.grid[position.y][position.x] + self.cost;
        let last_direction_times = if self.last_direction == direction {
            self.last_direction_times + 1
        } else {
            1
        };
        let path = Path {
            cost,
            position,
            last_direction: direction,
            last_direction_times,
        };
        return path;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
}
