use std::collections::{HashMap, HashSet};

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = count_flips(input);
    return result;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let floor = live_floor(input, 100);
    return floor.count_black() as i64;
}

#[derive(Clone)]
struct Floor {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,

    black_tiles: HashMap<i32, HashSet<i32>>,
}

impl Floor {
    fn count_black(&self) -> usize {
        let mut count: usize = 0;
        for row in self.black_tiles.values() {
            count += row.len();
        }

        return count;
    }

    fn live_day(&mut self) {
        let mut next_day_floor = self.clone();
        for index_y in self.min_y - 1..self.max_y + 2 {
            for index_x in self.min_x - 1..self.max_x + 2 {
                let surrounding = self.count_black_around(index_x, index_y);
                match self.is_black(index_x, index_y) {
                    true => {
                        if surrounding == 0 || surrounding > 2 {
                            next_day_floor
                                .black_tiles
                                .get_mut(&index_y)
                                .unwrap()
                                .remove(&index_x);
                        }
                    }
                    false => {
                        if surrounding == 2 {
                            match next_day_floor.black_tiles.get_mut(&index_y) {
                                None => {
                                    let mut row = HashSet::new();
                                    row.insert(index_x);
                                    next_day_floor.black_tiles.insert(index_y, row);

                                    if index_y > next_day_floor.max_y {
                                        next_day_floor.max_y = index_y;
                                    } else if index_y < next_day_floor.min_y {
                                        next_day_floor.min_y = index_y;
                                    }
                                }
                                Some(row) => {
                                    row.insert(index_x);
                                }
                            }

                            if index_x > next_day_floor.max_x {
                                next_day_floor.max_x = index_x;
                            } else if index_x < next_day_floor.min_x {
                                next_day_floor.min_x = index_x;
                            }
                        }
                    }
                }
            }
        }

        *self = next_day_floor;
    }

    #[allow(dead_code)]
    fn print(&self) {
        for index_y in self.min_y..self.max_y + 1 {
            let mut row = String::new();
            for index_x in self.min_x..self.max_x + 1 {
                if self.is_black(index_x, index_y) {
                    row += " #";
                } else {
                    row += " .";
                }
            }
            println!("{}", row);
        }
        println!();
    }

    fn count_black_around(&self, x: i32, y: i32) -> usize {
        let mut count = 0;

        //   x-1,y-1  x,y-1
        // x-1,y  x,y  x+1,y
        //   x,y+1  x+1,y+1

        if self.is_black(x - 1, y - 1) {
            count += 1;
        }
        if self.is_black(x, y - 1) {
            count += 1;
        }
        if self.is_black(x - 1, y) {
            count += 1;
        }
        // if self.is_black(x, y) {
        //     count += 1;
        // }
        if self.is_black(x + 1, y) {
            count += 1;
        }
        if self.is_black(x, y + 1) {
            count += 1;
        }
        if self.is_black(x + 1, y + 1) {
            count += 1;
        }

        return count;
    }

    fn is_black(&self, x: i32, y: i32) -> bool {
        match self.black_tiles.get(&y) {
            None => false,
            Some(row) => row.contains(&x),
        }
    }

    fn from(tiles: &Vec<Tile>) -> Floor {
        let mut floor = Self {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            black_tiles: HashMap::new(),
        };

        for tile in tiles.iter() {
            match floor.black_tiles.get_mut(&tile.y) {
                None => {
                    let mut row = HashSet::new();
                    row.insert(tile.x);
                    floor.black_tiles.insert(tile.y, row);

                    if tile.y > floor.max_y {
                        floor.max_y = tile.y;
                    } else if tile.y < floor.min_y {
                        floor.min_y = tile.y;
                    }
                }
                Some(y) => {
                    if y.contains(&tile.x) {
                        y.remove(&tile.x);
                    } else {
                        y.insert(tile.x);
                    }
                }
            }

            if tile.x > floor.max_x {
                floor.max_x = tile.x;
            } else if tile.x < floor.min_x {
                floor.min_x = tile.x;
            }
        }

        return floor;
    }
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
    let floor = Floor::from(&tiles);
    return floor.count_black() as i64;
}

fn live_floor(input: &str, days: usize) -> Floor {
    let tiles = parse_input(input);
    let mut floor = Floor::from(&tiles);
    // floor.print();
    for _day in 0..days {
        floor.live_day();
        // println!("Day {}: {}", _day + 1, floor.count_black());
        // floor.print();
    }

    return floor;
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

    #[test]
    fn example_live_1_day_count() {
        let input = get_example_input();
        let floor = live_floor(input, 1);

        assert_eq!(15, floor.count_black());
    }

    #[test]
    fn example_live_10_days_count() {
        let input = get_example_input();
        let floor = live_floor(input, 10);

        assert_eq!(37, floor.count_black());
    }

    #[test]
    fn example_live_100_days_count() {
        let input = get_example_input();
        let floor = live_floor(input, 100);

        assert_eq!(2208, floor.count_black());
    }

    #[test]
    fn part2_result() {
        let result = get_part2_result();

        assert_eq!(3445, result);
    }
}
