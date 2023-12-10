use std::collections::{HashMap, LinkedList};

fn main() {
    let input = get_input();
    let map = parse_input(input);

    let result_part1 = largest_distance(&map);
    println!("Part1: {}", result_part1);

    let result_part2 = enclosed_tiles(&map);
    println!("Part2: {}", result_part2);
}

fn enclosed_tiles(map: &Map) -> u32 {
    let width = *map.map.keys().max().unwrap();
    let height = *map.map.values().map(|column| column.keys().max().unwrap()).max().unwrap();

    let mut walk_map = vec![vec![false; (width + 2) * 2]; (height + 2) * 2];
    map.identify_loop(|pos, direction| {
        let pos = Position {
            x: pos.x * 2 + 1,
            y: pos.y * 2 + 1,
        };
        walk_map[pos.y][pos.x] = true;
        let pos = pos.clone().move_by(direction.opposite()).unwrap();
        walk_map[pos.y][pos.x] = true;
    });

    flood_fill(&mut walk_map);

    let mut enclosed = 0;
    for y in 0..height {
        let y = y * 2 + 1;
        for x in 0..width {
            let x = x * 2 + 1;
            if !walk_map[y][x] {
                enclosed += 1;
            }
        }
    }
    return enclosed;
}

fn flood_fill(map: &mut Vec<Vec<bool>>) {
    let heigt = map.len();
    let width = map[0].len();

    let mut to_go = LinkedList::new();
    to_go.push_back(Position { x: 0, y: 0 });
    
    while let Some(pos) = to_go.pop_front() {
        if pos.x >= width || pos.y >= heigt {
            continue;
        }
        if map[pos.y][pos.x] {
            continue;
        }

        map[pos.y][pos.x] = true;
        if let Some(pos) = pos.move_by(Direction::Left) {
            to_go.push_back(pos);
        }
        if let Some(pos) = pos.move_by(Direction::Right) {
            to_go.push_back(pos);
        }
        if let Some(pos) = pos.move_by(Direction::Up) {
            to_go.push_back(pos);
        }
        if let Some(pos) = pos.move_by(Direction::Down) {
            to_go.push_back(pos);
        }
    }
}

fn largest_distance(map: &Map) -> u32 {
    let mut distance = 0;
    map.identify_loop(|_, _| distance += 1);
    return distance / 2;
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

    fn identify_loop(&self, mut callback: impl FnMut(&Position, &Direction)) -> u32 {
        let mut distance = 0;
        let mut position = self.start.unwrap();
        let mut last_direction = None;
        while distance == 0 || position != self.start.unwrap() {
            let (position_new, last_direction_new) = self.make_a_move(&position, last_direction);
            position = position_new;
            last_direction = Some(last_direction_new);
            callback(&position, &last_direction_new);
            distance += 1;
        }
    
        return distance;
    }
    
    fn make_a_move(
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
            if let Some(position) = position.move_by(direction) {
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
    fn move_by(&self, direction: Direction) -> Option<Self> {
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

    fn get_example1_part1_input() -> &'static str {
        include_str!("./example1_part1.txt")
    }

    fn get_example2_part1_input() -> &'static str {
        include_str!("./example2_part1.txt")
    }

    fn get_example3_part1_input() -> &'static str {
        include_str!("./example3_part1.txt")
    }

    fn get_example4_part1_input() -> &'static str {
        include_str!("./example4_part1.txt")
    }

    #[test]
    fn part1_example1() {
        let input = get_example1_part1_input();
        let map = parse_input(input);
        let result = largest_distance(&map);
        assert_eq!(result, 4);
    }

    #[test]
    fn part1_example2() {
        let input = get_example2_part1_input();
        let map = parse_input(input);
        let result = largest_distance(&map);
        assert_eq!(result, 4);
    }

    #[test]
    fn part1_example3() {
        let input = get_example3_part1_input();
        let map = parse_input(input);
        let result = largest_distance(&map);
        assert_eq!(result, 8);
    }

    #[test]
    fn part1_example4() {
        let input = get_example4_part1_input();
        let map = parse_input(input);
        let result = largest_distance(&map);
        assert_eq!(result, 8);
    }

    fn get_example1_part2_input() -> &'static str {
        include_str!("./example1_part2.txt")
    }

    fn get_example2_part2_input() -> &'static str {
        include_str!("./example2_part2.txt")
    }

    fn get_example3_part2_input() -> &'static str {
        include_str!("./example3_part2.txt")
    }

    fn get_example4_part2_input() -> &'static str {
        include_str!("./example4_part2.txt")
    }

    #[test]
    fn part2_example1() {
        let input = get_example1_part2_input();
        let map = parse_input(input);
        let result = enclosed_tiles(&map);
        assert_eq!(result, 4);
    }

    #[test]
    fn part2_example2() {
        let input = get_example2_part2_input();
        let map = parse_input(input);
        let result = enclosed_tiles(&map);
        assert_eq!(result, 4);
    }

    #[test]
    fn part2_example3() {
        let input = get_example3_part2_input();
        let map = parse_input(input);
        let result = enclosed_tiles(&map);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2_example4() {
        let input = get_example4_part2_input();
        let map = parse_input(input);
        let result = enclosed_tiles(&map);
        assert_eq!(result, 10);
    }
}
