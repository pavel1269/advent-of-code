use std::collections::HashSet;

fn main() {
    let input = get_input();
    let result_part1 = part1(&input);
    println!("Part1: {}", result_part1);
}

fn part1(input: &str) -> usize {
    let map = Map::from(input);
    let paths = map.list_paths();
    return paths.into_iter().max().unwrap() - 1;
}

#[derive(Debug, Clone)]
struct Path {
    visited: HashSet<Position>,
    at: Position,
}

impl Path {
    fn move_to(&mut self, pos: &Position) {
        self.at = *pos;
        self.visited.insert(*pos);
    }

    fn visited(&self, pos: &Position) -> bool {
        self.visited.contains(pos)
    }

    fn new(pos: Position) -> Self {
        Self {
            visited: HashSet::from([pos]),
            at: pos,
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn list_paths(&self) -> Vec<usize> {
        let start = self.determine_start();
        let end = self.determine_end();
        let max = Position {
            x: self.tiles.first().unwrap().len(),
            y: self.tiles.len(),
        };
        let mut paths = vec![Path::new(start)];
        let mut final_paths = Vec::new();
        while let Some(path) = paths.pop() {
            for path in path
                .at
                .next_moves(&max)
                .into_iter()
                .filter(|movement| {
                    return !path.visited(&movement.pos) && self.can_go_from(&path.at, movement);
                })
                .map(|movement| {
                    let mut path = path.clone();
                    path.move_to(&movement.pos);
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

        return final_paths.into_iter().map(|path| path.visited.len()).collect();
    }

    fn can_go_from(&self, at: &Position, to_move: &Movement) -> bool {
        let tile_to = &self.tiles[to_move.pos.y][to_move.pos.x];
        if tile_to == &Tile::Forest {
            return false;
        }
        let tile_at = &self.tiles[at.y][at.x];
        
        return match tile_at {
            Tile::Forest => panic!(),
            Tile::Path => true,
            Tile::Slope(dir) => {
                return &to_move.dir == dir;
            },
        }
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
            let movement = Movement { pos, dir: Direction::Left, };
            next_moves.push(movement);
        }
        if self.y > 0 {
            let pos = Self {
                x: self.x,
                y: self.y - 1,
            };
            let movement = Movement { pos, dir: Direction::Up, };
            next_moves.push(movement);
        }
        if self.x < max.x - 1 {
            let pos = Self {
                x: self.x + 1,
                y: self.y,
            };
            let movement = Movement { pos, dir: Direction::Right, };
            next_moves.push(movement);
        }
        if self.y < max.y - 1 {
            let pos = Self {
                x: self.x,
                y: self.y + 1,
            };
            let movement = Movement { pos, dir: Direction::Down, };
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
}
