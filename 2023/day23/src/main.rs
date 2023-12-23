use std::collections::{HashMap, HashSet};

fn main() {
    let input = get_input();
    let result_part1 = part1(&input);
    println!("Part1: {}", result_part1);
    let result_part2 = part2(&input);
    println!("Part2: {}", result_part2);
}

fn part1(input: &str) -> usize {
    let map = Map::from(input);
    let paths = map.list_path_lengths(false);
    return paths.into_iter().max().unwrap() - 1;
}

fn part2(input: &str) -> usize {
    let map = Map::from(input);
    let paths = map.list_path_lengths(true);
    return paths.into_iter().max().unwrap() - 1;
}

#[derive(Debug, Clone)]
struct Path {
    visited: HashSet<Position>,
    at: Position,
    cost: usize,
}

impl Path {
    fn move_to(&mut self, pos: &Position, cost: usize) {
        self.at = *pos;
        self.visited.insert(*pos);
        self.cost += cost;
    }

    fn visited(&self, pos: &Position) -> bool {
        self.visited.contains(pos)
    }

    fn new(pos: Position) -> Self {
        Self {
            visited: HashSet::from([pos]),
            at: pos,
            cost: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct PathWithSlopes {
    visited: HashSet<Position>,
    start: Position,
    at: Position,
    initial_dir: Direction,
    last_dir: Direction,
    slope: Slope,
}

impl PathWithSlopes {
    fn move_to(&mut self, movement: &Movement) {
        self.at = movement.pos;
        self.visited.insert(movement.pos);
        self.last_dir = movement.dir;
        self.slope = movement.slope.merge(self.slope);
    }

    fn visited(&self, pos: &Position) -> bool {
        self.visited.contains(pos)
    }

    fn new(pos: Position, dir: Direction) -> Self {
        Self {
            visited: HashSet::from([pos]),
            start: pos,
            at: pos,
            initial_dir: dir,
            last_dir: Direction::Down,
            slope: Slope::None,
        }
    }

    fn new_at(start: Position, movement: &Movement) -> Self {
        Self {
            visited: HashSet::from([start, movement.pos]),
            start: start,
            at: movement.pos,
            initial_dir: movement.dir,
            last_dir: movement.dir,
            slope: movement.slope,
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn build_connection_graph(
        &self,
        ignore_slopes: bool,
    ) -> HashMap<Position, HashMap<Direction, Option<(Position, usize)>>> {
        let start = self.determine_start();
        let end = self.determine_end();
        let max = Position {
            x: self.tiles.first().unwrap().len(),
            y: self.tiles.len(),
        };
        let mut paths = vec![PathWithSlopes::new(start, Direction::Down)];
        let mut crossings = HashMap::from([
            (
                start,
                HashMap::from([(Direction::Down, None as Option<(Position, usize)>)]),
            ),
            (
                end,
                HashMap::from([(Direction::Up, None as Option<(Position, usize)>)]),
            ),
        ]);
        while let Some(mut path) = paths.pop() {
            let all_next_moves = path
                .at
                .next_moves(&max)
                .into_iter()
                .filter(|movement| self.can_go_from(&path.at, movement, ignore_slopes))
                .map(|mut movement| {
                    self.mark_slope_movement(&path.at, &mut movement);
                    return movement;
                })
                .collect::<Vec<_>>();
            let next_moves = all_next_moves
                .iter()
                .filter(|movement| !path.visited(&movement.pos))
                .collect::<Vec<_>>();
            if next_moves.len() == 0 {
                if path.at == end {
                    if ignore_slopes || path.slope.is_flat_or_down() {
                        let source_crossing = crossings
                            .iter_mut()
                            .filter(|(pos, _)| pos == &&path.start)
                            .map(|(_, crossing)| crossing)
                            .next()
                            .unwrap();
                        *source_crossing.get_mut(&path.initial_dir).unwrap() =
                            Some((path.at, path.visited.len() - 1));
                    }

                    if ignore_slopes || path.slope.is_flat_or_up() {
                        let dest_crossing = crossings.get_mut(&path.at).unwrap();
                        *dest_crossing.get_mut(&path.last_dir.opposite()).unwrap() =
                            Some((path.start, path.visited.len() - 1));
                    }
                }
            } else if next_moves.len() == 1 {
                path.move_to(&next_moves.first().unwrap());
                paths.push(path);
            } else {
                if ignore_slopes || path.slope.is_flat_or_down() {
                    let source_crossing = crossings
                        .iter_mut()
                        .filter(|(pos, _)| pos == &&path.start)
                        .map(|(_, crossing)| crossing)
                        .next()
                        .unwrap();
                    *source_crossing.get_mut(&path.initial_dir).unwrap() =
                        Some((path.at, path.visited.len() - 1));
                }

                if crossings.contains_key(&path.at) {
                    if ignore_slopes || path.slope.is_flat_or_up() {
                        let dest_crossing = crossings.get_mut(&path.at).unwrap();
                        *dest_crossing.get_mut(&path.last_dir.opposite()).unwrap() =
                            Some((path.start, path.visited.len() - 1));
                    }
                } else {
                    crossings.insert(
                        path.at,
                        all_next_moves
                            .iter()
                            .map(|movement| (movement.dir, None))
                            .collect(),
                    );

                    if ignore_slopes || path.slope.is_flat_or_up() {
                        let dest_crossing = crossings.get_mut(&path.at).unwrap();
                        *dest_crossing.get_mut(&path.last_dir.opposite()).unwrap() =
                            Some((path.start, path.visited.len() - 1));
                    }

                    let mut new_paths = next_moves
                        .iter()
                        .map(|movement| PathWithSlopes::new_at(path.at, movement))
                        .collect();
                    paths.append(&mut new_paths);
                }
            }
        }

        return crossings;
    }

    fn list_path_lengths(&self, ignore_slopes: bool) -> Vec<usize> {
        let start = self.determine_start();
        let end = self.determine_end();
        let connections = self.build_connection_graph(ignore_slopes);

        let mut paths = vec![Path::new(start)];
        let mut final_paths = Vec::new();
        while let Some(path) = paths.pop() {
            let targets = connections.get(&path.at).unwrap();
            for path in targets
                .values()
                .filter(|target| target.is_some())
                .map(|target| target.unwrap())
                .filter(|(target, _)| !path.visited(target))
                .map(|(target, cost)| {
                    let mut path = path.clone();
                    path.move_to(&target, cost);
                    return path;
                })
            {
                if path.at == end {
                    final_paths.push(path);
                } else {
                    paths.push(path);
                }
            }
        }

        return final_paths.into_iter().map(|path| path.cost + 1).collect();
    }

    fn can_go_from(&self, at: &Position, to_move: &Movement, ignore_slopes: bool) -> bool {
        let tile_to = &self.tiles[to_move.pos.y][to_move.pos.x];
        if tile_to == &Tile::Forest {
            return false;
        }

        let tile_at = &self.tiles[at.y][at.x];
        return match tile_at {
            Tile::Forest => panic!(),
            Tile::Path => true,
            Tile::Slope(dir) => ignore_slopes || &to_move.dir == dir,
        };
    }

    fn mark_slope_movement(&self, at: &Position, to_move: &mut Movement) {
        let tile_at = &self.tiles[at.y][at.x];
        match tile_at {
            Tile::Slope(dir) => {
                let downward = &to_move.dir == dir;
                if downward {
                    to_move.slope = Slope::Downward;
                } else {
                    to_move.slope = Slope::Upward;
                }
            }
            _ => (),
        };
    }

    fn determine_start(&self) -> Position {
        self.determine_first_path(0)
    }

    fn determine_end(&self) -> Position {
        self.determine_first_path(self.tiles.len() - 1)
    }

    fn determine_first_path(&self, y: usize) -> Position {
        let x = self
            .tiles
            .get(y)
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile == &&Tile::Path)
            .map(|(index, _)| index)
            .next()
            .unwrap();
        return Position { x, y };
    }

    fn from(str: &str) -> Self {
        let tiles = str
            .lines()
            .enumerate()
            .map(|(_, line)| {
                line.chars()
                    .enumerate()
                    .map(|(_, char)| Tile::from(char).unwrap())
                    .collect()
            })
            .collect();
        let result = Self { tiles };
        return result;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Movement {
    pos: Position,
    dir: Direction,
    slope: Slope,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Slope {
    None,
    Upward,
    Downward,
    Both,
}

impl Slope {
    fn merge(&self, other: Self) -> Self {
        match self {
            Self::None => other,
            Self::Upward => match other {
                Self::None => Self::Upward,
                Self::Upward => Self::Upward,
                Self::Downward => Self::Both,
                Self::Both => Self::Both,
            },
            Self::Downward => match other {
                Self::None => Self::Downward,
                Self::Upward => Self::Both,
                Self::Downward => Self::Downward,
                Self::Both => Self::Both,
            },
            Self::Both => Self::Both,
        }
    }

    fn is_flat_or_down(&self) -> bool {
        match self {
            Self::None => true,
            Self::Upward => false,
            Self::Downward => true,
            Self::Both => false,
        }
    }

    fn is_flat_or_up(&self) -> bool {
        match self {
            Self::None => true,
            Self::Upward => true,
            Self::Downward => false,
            Self::Both => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn next_moves(&self, max: &Position) -> Vec<Movement> {
        let mut next_moves = Vec::new();
        if self.x > 0 {
            let pos = Self {
                x: self.x - 1,
                y: self.y,
            };
            let movement = Movement {
                pos,
                dir: Direction::Left,
                slope: Slope::None,
            };
            next_moves.push(movement);
        }
        if self.y > 0 {
            let pos = Self {
                x: self.x,
                y: self.y - 1,
            };
            let movement = Movement {
                pos,
                dir: Direction::Up,
                slope: Slope::None,
            };
            next_moves.push(movement);
        }
        if self.x < max.x - 1 {
            let pos = Self {
                x: self.x + 1,
                y: self.y,
            };
            let movement = Movement {
                pos,
                dir: Direction::Right,
                slope: Slope::None,
            };
            next_moves.push(movement);
        }
        if self.y < max.y - 1 {
            let pos = Self {
                x: self.x,
                y: self.y + 1,
            };
            let movement = Movement {
                pos,
                dir: Direction::Down,
                slope: Slope::None,
            };
            next_moves.push(movement);
        }
        return next_moves;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Forest,
    Path,
    Slope(Direction),
}

impl Tile {
    fn from(char: char) -> Option<Self> {
        match char {
            '#' => Some(Self::Forest),
            '.' => Some(Self::Path),
            _ => {
                if let Some(slope) = Direction::from(char) {
                    return Some(Self::Slope(slope));
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
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn from(char: char) -> Option<Self> {
        match char {
            '^' => Some(Self::Up),
            'v' => Some(Self::Down),
            '<' => Some(Self::Left),
            '>' => Some(Self::Right),
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

    fn get_example_input() -> &'static str {
        include_str!("./example.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = part1(&input);
        assert_eq!(result, 94);
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = part2(&input);
        assert_eq!(result, 154);
    }
}
