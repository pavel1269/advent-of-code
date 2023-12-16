use std::collections::{HashMap, HashSet};

fn main() {
    let input = get_input();
    let result_part1 = part1(input);
    println!("Part1: {}", result_part1);
    let result_part2 = part2(input);
    println!("Part2: {}", result_part2);
}

fn part1(input: &str) -> usize {
    let map = Map::from(input);
    let beam = Beam {
        position: Position { x: 0, y: 0 },
        direction: Direction::Right,
    };
    let result = map.count_visited(beam);
    return result;
}

fn part2(input: &str) -> usize {
    let map = Map::from(input);
    let map_width = map.mirrors_columns.len();
    let map_height = map.mirrors_rows.len();
    let mut max = usize::MIN;
    for x in 0..map_width {
        let beam = Beam {
            position: Position { x, y: 0 },
            direction: Direction::Down,
        };
        let result = map.count_visited(beam);
        max = max.max(result);

        let beam = Beam {
            position: Position {
                x,
                y: map_height - 1,
            },
            direction: Direction::Up,
        };
        let result = map.count_visited(beam);
        max = max.max(result);
    }

    for y in 0..map_height {
        let beam = Beam {
            position: Position { x: 0, y },
            direction: Direction::Right,
        };
        let result = map.count_visited(beam);
        max = max.max(result);

        let beam = Beam {
            position: Position {
                x: map_width - 1,
                y,
            },
            direction: Direction::Left,
        };
        let result = map.count_visited(beam);
        max = max.max(result);
    }

    return max;
}

struct Visited {
    map: Vec<Vec<bool>>,
}

impl Visited {
    fn mark(&mut self, position: &Position) {
        self.map[position.y][position.x] = true;
    }

    fn count(&self) -> usize {
        self.map
            .iter()
            .map(|col| col.iter().filter(|&&f| f).count())
            .sum()
    }

    fn new(map: &Map) -> Self {
        Visited {
            map: vec![vec![false; map.mirrors_columns.len()]; map.mirrors_rows.len()],
        }
    }
}

#[derive(Debug)]
struct Map {
    mirrors_columns: Vec<HashMap<usize, Mirror>>,
    mirrors_rows: Vec<HashMap<usize, Mirror>>,
}

impl Map {
    fn count_visited(&self, beam: Beam) -> usize {
        let mut visited = Visited::new(self);
        visited.mark(&beam.position);
        let mut beams = vec![beam];
        let mut cache = HashSet::new();
        while let Some(mut beam) = beams.pop() {
            if cache.contains(&beam) {
                continue;
            } else {
                cache.insert(beam.clone());
            }
            let next_target = self.get_next_mirror(&beam.position, beam.direction);
            match beam.step_towards_target(self, next_target) {
                StepResult::Ok => {
                    visited.mark(&beam.position);
                    beams.push(beam);
                }
                StepResult::MapEdge => {}
                StepResult::Target => {
                    visited.mark(&beam.position);
                    let mut beams_new = beam.reflect(self);
                    beams.append(&mut beams_new);
                }
            };
        }

        let visited = visited.count();
        return visited;
    }

    fn get_mirror(&self, position: &Position) -> Option<Mirror> {
        self.mirrors_rows[position.y].get(&position.x).copied()
    }

    fn get_next_mirror(&self, from: &Position, direction: Direction) -> Option<Position> {
        match direction {
            Direction::Up => {
                let x = from.x;
                let mut mirrors = self.mirrors_columns[from.x]
                    .keys()
                    .copied()
                    .filter(|&y| y < from.y)
                    .collect::<Vec<_>>();
                mirrors.sort();
                if let Some(y) = mirrors.last().copied() {
                    return Some(Position { x, y });
                }
                return None;
            }
            Direction::Down => {
                let x = from.x;
                let mut mirrors = self.mirrors_columns[from.x]
                    .keys()
                    .copied()
                    .filter(|&y| y > from.y)
                    .collect::<Vec<_>>();
                mirrors.sort();
                if let Some(y) = mirrors.first().copied() {
                    return Some(Position { x, y });
                }
                return None;
            }
            Direction::Right => {
                let y = from.y;
                let mut mirrors = self.mirrors_rows[from.y]
                    .keys()
                    .copied()
                    .filter(|&x| x > from.x)
                    .collect::<Vec<_>>();
                mirrors.sort();
                if let Some(x) = mirrors.first().copied() {
                    return Some(Position { x, y });
                }
                return None;
            }
            Direction::Left => {
                let y = from.y;
                let mut mirrors = self.mirrors_rows[from.y]
                    .keys()
                    .copied()
                    .filter(|&x| x < from.x)
                    .collect::<Vec<_>>();
                mirrors.sort();
                if let Some(x) = mirrors.last().copied() {
                    return Some(Position { x, y });
                }
                return None;
            }
        }
    }

    fn from(input: &str) -> Self {
        let mut mirrors_columns = Vec::new();
        let mut mirrors_rows = Vec::new();
        for (index_row, line) in input.lines().enumerate() {
            mirrors_rows.push(HashMap::new());
            if mirrors_columns.len() == 0 {
                mirrors_columns = vec![HashMap::new(); line.len()];
            }
            for (index_column, char) in line.chars().enumerate() {
                if let Some(mirror) = Mirror::from(char) {
                    mirrors_rows[index_row].insert(index_column, mirror);
                    mirrors_columns[index_column].insert(index_row, mirror);
                }
            }
        }

        let map = Map {
            mirrors_columns,
            mirrors_rows,
        };
        return map;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mirror {
    Vertical,
    Horizontal,
    LeaningRight,
    LeaningLeft,
}

impl Mirror {
    fn reflect(&self, direction: Direction) -> Vec<Direction> {
        match direction {
            Direction::Up => match self {
                Mirror::Vertical => vec![Direction::Up],
                Mirror::Horizontal => vec![Direction::Left, Direction::Right],
                Mirror::LeaningRight => vec![Direction::Right],
                Mirror::LeaningLeft => vec![Direction::Left],
            },
            Direction::Down => match self {
                Mirror::Vertical => vec![Direction::Down],
                Mirror::Horizontal => vec![Direction::Left, Direction::Right],
                Mirror::LeaningRight => vec![Direction::Left],
                Mirror::LeaningLeft => vec![Direction::Right],
            },
            Direction::Left => match self {
                Mirror::Vertical => vec![Direction::Up, Direction::Down],
                Mirror::Horizontal => vec![Direction::Left],
                Mirror::LeaningRight => vec![Direction::Down],
                Mirror::LeaningLeft => vec![Direction::Up],
            },
            Direction::Right => match self {
                Mirror::Vertical => vec![Direction::Up, Direction::Down],
                Mirror::Horizontal => vec![Direction::Right],
                Mirror::LeaningRight => vec![Direction::Up],
                Mirror::LeaningLeft => vec![Direction::Down],
            },
        }
    }

    fn from(char: char) -> Option<Self> {
        match char {
            '.' => None,
            '|' => Some(Self::Vertical),
            '-' => Some(Self::Horizontal),
            '/' => Some(Self::LeaningRight),
            '\\' => Some(Self::LeaningLeft),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Beam {
    position: Position,
    direction: Direction,
}

impl Beam {
    fn reflect(&self, map: &Map) -> Vec<Beam> {
        let mirror = map.get_mirror(&self.position).unwrap();
        let beams = mirror
            .reflect(self.direction)
            .iter()
            .copied()
            .map(|direction| Beam {
                position: self.position.clone(),
                direction,
            })
            .collect();
        return beams;
    }

    fn step_towards_target(&mut self, map: &Map, target: Option<Position>) -> StepResult {
        if let Some(pos) = self.position.step(self.direction, map) {
            self.position = pos;
            if let Some(target) = target {
                if target == self.position {
                    return StepResult::Target;
                }
            }
            return StepResult::Ok;
        }
        return StepResult::MapEdge;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum StepResult {
    Ok,
    Target,
    MapEdge,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn step(&self, direction: Direction, map: &Map) -> Option<Self> {
        match direction {
            Direction::Up => {
                let x = self.x;
                if let Some(y) = self.y.checked_add_signed(-1) {
                    return Some(Position { x, y });
                }
                return None;
            }
            Direction::Down => {
                let x = self.x;
                let y = self.y + 1;
                if y >= map.mirrors_rows.len() {
                    return None;
                }
                return Some(Position { x, y });
            }
            Direction::Right => {
                let x = self.x + 1;
                let y = self.y;
                if x >= map.mirrors_columns.len() {
                    return None;
                }
                return Some(Position { x, y });
            }
            Direction::Left => {
                let y = self.y;
                if let Some(x) = self.x.checked_add_signed(-1) {
                    return Some(Position { x, y });
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

    #[test_case(3, 4, Direction::Up, None)]
    #[test_case(3, 4, Direction::Down, Some((3, 7)))]
    #[test_case(0, 5, Direction::Up, Some((0, 1)))]
    #[test_case(0, 5, Direction::Down, None)]
    #[test_case(4, 5, Direction::Left, None)]
    #[test_case(4, 5, Direction::Right, Some((9, 5)))]
    #[test_case(9, 3, Direction::Left, Some((8, 3)))]
    #[test_case(9, 3, Direction::Right, None)]
    #[test_case(1, 0, Direction::Right, Some((5, 0)))]
    #[test_case(1, 0, Direction::Left, None)]
    #[test_case(0, 1, Direction::Up, None)]
    #[test_case(0, 1, Direction::Down, None)]
    #[test_case(5, 3, Direction::Up, Some((5, 2)))]
    #[test_case(1, 1, Direction::Down, Some((1, 7)))]
    #[test_case(1, 2, Direction::Right, Some((5, 2)))]
    #[test_case(8, 2, Direction::Left, Some((6, 2)))]
    fn map_get_next_mirror(
        x: usize,
        y: usize,
        direction: Direction,
        expected: Option<(usize, usize)>,
    ) {
        let input = get_example_input();
        let map: Map = Map::from(input);
        let result = map.get_next_mirror(&Position { x, y }, direction);
        if let Some((x, y)) = expected {
            let pos = Some(Position { x, y });
            assert_eq!(result, pos);
        } else {
            assert_eq!(result, None);
        }
    }

    #[test_case(0, None, StepResult::Ok)]
    #[test_case(1, None, StepResult::MapEdge)]
    #[test_case(0, Some((1, 1)), StepResult::Ok)]
    #[test_case(0, Some((1, 0)), StepResult::Target)]
    fn beam_step_towards_target(x: usize, target: Option<(usize, usize)>, expected: StepResult) {
        let mut beam = Beam {
            direction: Direction::Right,
            position: Position { x, y: 0 },
        };
        let map = Map {
            mirrors_columns: vec![HashMap::new(); 2],
            mirrors_rows: vec![HashMap::new(); 2],
        };
        let result = if let Some((x, y)) = target {
            beam.step_towards_target(&map, Some(Position { x, y }))
        } else {
            beam.step_towards_target(&map, None)
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = part1(input);
        assert_eq!(result, 46);
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = part2(input);
        assert_eq!(result, 51);
    }
}
