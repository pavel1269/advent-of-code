use std::collections::HashSet;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = count_flips(input);
    return result;
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Tile {
    x: i32,
    y: i32,
}

impl Default for Tile {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(PartialEq)]
enum Direction {
    None,

    North,
    South,
}

fn count_flips(input: &str) -> i64 {
    let tiles = parse_input(input);
    let mut flips: HashSet<Tile> = HashSet::new();
    for tile in tiles.iter() {
        if flips.contains(tile) {
            flips.remove(tile);
        } else {
            flips.insert(*tile);
        }
    }

    return flips.len() as i64;
}

fn parse_input(input: &str) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();
    for line in input.lines() {
        let tile = parse_tile(line);
        tiles.push(tile);
    }
    return tiles;
}

fn parse_tile(input: &str) -> Tile {
    let mut tile = Tile::default();
    let mut last_direction = Direction::None;
    for char in input.chars() {
        match char {
            'n' => {
                if last_direction != Direction::None {
                    panic!();
                }
                last_direction = Direction::North;
            }
            's' => {
                if last_direction != Direction::None {
                    panic!();
                }
                last_direction = Direction::South;
            }
            'e' => {
                match last_direction {
                    Direction::None => {
                        tile.x += 1;
                    }
                    Direction::South => {
                        tile.y += 1;
                        tile.x += 1;
                    }
                    Direction::North => {
                        tile.y -= 1;
                    }
                }
                last_direction = Direction::None;
            }
            'w' => {
                match last_direction {
                    Direction::None => {
                        tile.x -= 1;
                    }
                    Direction::South => {
                        tile.y += 1;
                    }
                    Direction::North => {
                        tile.x -= 1;
                        tile.y -= 1;
                    }
                }
                last_direction = Direction::None;
            }
            _ => {
                panic!();
            }
        }
    }

    return tile;
}

fn get_challenge_input() -> &'static str {
    include_str!("input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        include_str!("example.txt")
    }

    #[test]
    fn example_flips_count() {
        let input = get_example_input();
        let result = count_flips(input);

        assert_eq!(10, result);
    }

    #[test]
    fn part1_flips_count() {
        let result = get_part1_result();

        assert_eq!(282, result);
    }
}
