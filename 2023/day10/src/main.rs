use std::collections::HashMap;

fn main() {
    let input = get_input();
    let map = parse_input(input);

    let result_part1 = largest_distance(&map);
    println!("Part1: {}", result_part1);
}

fn largest_distance(map: &Map) -> u32 {
    let loop_size = loop_size(map);
    return loop_size / 2;
}

fn loop_size(map: &Map) -> u32 {
    let mut distance = 0;
    let mut position = map.start.unwrap();
    let mut last_direction = None;
    while distance == 0 || position != map.start.unwrap() {
        let (position_new, last_direction_new) = map.r#move(&position, last_direction);
        position = position_new;
        last_direction = Some(last_direction_new);
        distance += 1;
    }

    return distance;
}

#[derive(Debug)]
struct Map {
    map: HashMap<usize, HashMap<usize, Pipe>>,
    start: Option<Position>,
}

impl Map {
    fn new() -> Self {
        Map {
            map: HashMap::new(),
            start: None,
        }
    }

    fn r#move(
        &self,
        position: &Position,
        last_direction: Option<Direction>,
    ) -> (Position, Direction) {
        let directions = vec![
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ];

        for direction in directions {
            if let Some(res) = self.try_move(position, last_direction, direction) {
                return res;
            }
        }

        panic!();
    }

    fn try_move(&self, position: &Position, last_direction: Option<Direction>, direction: Direction) -> Option<(Position, Direction)> {
        let pipe = self.get(position).unwrap();
        if last_direction != Some(direction.opposite()) && pipe.can_go(&direction) {
            if let Some(position) = position.r#move(direction) {
                if let Some(pipe) = self.get(&position) {
                    if pipe.can_go(&direction.opposite()) {
                        return Some((position, direction));
                    }
                }
            }
        }
        return None;
    }

    fn get(&self, position: &Position) -> Option<Pipe> {
        if self.map.contains_key(&position.x) {
            let map_x = self.map.get(&position.x).unwrap();
            if map_x.contains_key(&position.y) {
                return Some(map_x[&position.y]);
            }
        }
        return None;
    }

    fn add(&mut self, x: usize, y: usize, pipe: Pipe) {
        let x_entry = self.map.entry(x).or_insert(HashMap::new());
        x_entry.insert(y, pipe);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
}

impl Pipe {
    fn can_go(&self, direction: &Direction) -> bool {
        match direction {
            Direction::Left => match self {
                Self::Start | Self::Horizontal | Self::BendNW | Self::BendSW => true,
                _ => false,
            },
            Direction::Right => match self {
                Self::Start | Self::Horizontal | Self::BendNE | Self::BendSE => true,
                _ => false,
            },
            Direction::Up => match self {
                Self::Start | Self::Vertical | Self::BendNE | Self::BendNW => true,
                _ => false,
            },
            Direction::Down => match self {
                Self::Start | Self::Vertical | Self::BendSE | Self::BendSW => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn r#move(&self, direction: Direction) -> Option<Self> {
        let position: (Option<usize>, Option<usize>) = match direction {
            Direction::Down => (Some(self.x), Some(self.y + 1)),
            Direction::Up => (Some(self.x), self.y.checked_add_signed(-1)),
            Direction::Left => (self.x.checked_add_signed(-1), Some(self.y)),
            Direction::Right => (Some(self.x + 1), Some(self.y)),
        };

        if let Some(x) = position.0 {
            if let Some(y) = position.1 {
                return Some(Position { x, y });
            }
        }
        return None;
    }
}

fn parse_input(input: &str) -> Map {
    let mut map = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let pipe = match char {
                'S' => Some(Pipe::Start),
                '|' => Some(Pipe::Vertical),
                '-' => Some(Pipe::Horizontal),
                'L' => Some(Pipe::BendNE),
                'J' => Some(Pipe::BendNW),
                'F' => Some(Pipe::BendSE),
                '7' => Some(Pipe::BendSW),
                _ => None,
            };

            if let Some(pipe) = pipe {
                if pipe == Pipe::Start {
                    map.start = Some(Position { x, y });
                }
                map.add(x, y, pipe);
            }
        }
    }
    return map;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example1_input() -> &'static str {
        include_str!("./example1.txt")
    }

    fn get_example2_input() -> &'static str {
        include_str!("./example2.txt")
    }

    fn get_example3_input() -> &'static str {
        include_str!("./example3.txt")
    }

    fn get_example4_input() -> &'static str {
        include_str!("./example4.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example1_input();
        let map = parse_input(input);
        let result = largest_distance(&map);
        assert_eq!(result, 4);
    }

    #[test]
    fn part2_example() {
        let input = get_example2_input();
        let map = parse_input(input);
        let result = largest_distance(&map);
        assert_eq!(result, 4);
    }

    #[test]
    fn part3_example() {
        let input = get_example3_input();
        let map = parse_input(input);
        let result = largest_distance(&map);
        assert_eq!(result, 8);
    }

    #[test]
    fn part4_example() {
        let input = get_example4_input();
        let map = parse_input(input);
        let result = largest_distance(&map);
        assert_eq!(result, 8);
    }
}
