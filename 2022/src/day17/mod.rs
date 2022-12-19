use std::collections::{hash_map::Entry, HashMap};

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = fall_rocks(input, 2022);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = fall_rocks(input, 1000000000000);
    return result.to_string();
    // 1520579710189 - too low
    // 1520579710169 - too low
    // 1520000000026 - too low
}

fn fall_rocks(input: &str, rocks: u64) -> u64 {
    let directions = parse_input(input);
    let shapes = get_shapes();

    let mut map = Map::new();
    let mut shapes_iter = shapes.iter().cloned().enumerate().cycle();
    let mut directions_iter = directions.iter().cloned().enumerate().cycle();
    let cache_size = 200;
    let mut cache: HashMap<(Vec<Vec<bool>>, usize, usize), (u64, u64)> = HashMap::new();

    let mut rocks_dropped = 0;
    while rocks_dropped < rocks {
        if rocks_dropped % 10000 == 0 {
            println!("loop {}", rocks_dropped);
        }
        let (shape_index, mut shape) = shapes_iter.next().unwrap();
        let shape_start_y = map.get_height() + 3;
        shape.y = shape_start_y;
        loop {
            let (move_index, move_right) = directions_iter.next().unwrap();
            if shape.y == shape_start_y {
                match cache.entry((map.rocks.clone(), shape_index, move_index)) {
                    Entry::Occupied(entry) => {
                        let (cached_rocks, cached_height) = *entry.get();
                        let cached_height_move = map.get_height() - cached_height;
                        let cached_rocks_move = rocks_dropped - cached_rocks;
                        let cycles: u64 = (rocks - rocks_dropped) / cached_rocks_move;
                        if cycles > 0 {
                            println!("detected cycle at rocks: {}, height: {}", rocks_dropped, map.get_height());
                            map.height_offset += cached_height_move * cycles;
                            rocks_dropped += cached_rocks_move * cycles;
                            shape.y = map.get_height() + 3;
                            cache.clear();
                            println!("moved to rocks: {}, height: {}", rocks_dropped, map.get_height());
                        }
                    },
                    Entry::Vacant(entry) => {
                        entry.insert((rocks_dropped, map.get_height()));
                    },
                }
            }

            if move_right {
                map.try_move_right(&mut shape);
            }
            else {
                map.try_move_left(&mut shape);
            }

            if !map.move_shape_down(&mut shape, cache_size) {
                rocks_dropped += 1;
                break;
            }
        }
    }

    return map.get_height();
}

struct Map {
    rocks: Vec<Vec<bool>>,
    height_offset: u64,
}

impl Map {
    fn move_shape_down(&mut self, shape: &mut Shape, cache_size: usize) -> bool {
        for (x, y) in shape.points.iter() {
            let y = *y as u64 + shape.y;
            if y == 0 {
                self.add_shape(shape, cache_size);
                return false;
            }
            
            let x = x + shape.x;
            let y = y - 1;
            if self.is_occupied(x, y) {
                self.add_shape(shape, cache_size);
                return false;
            }
        }

        shape.y -= 1;
        return true;
    }

    fn add_shape(&mut self, shape: &Shape, cache_size: usize) {
        for (x, y) in shape.points.iter() {
            let x = x + shape.x;
            let y = *y as u64 + shape.y;
            self.add_point(x, y);
        }

        if self.rocks.len() > cache_size {
            let remove = self.rocks.len() - cache_size;
            self.rocks = self.rocks.iter().cloned().skip(remove).collect();
            self.height_offset += remove as u64;
        }
    }

    fn add_point(&mut self, x: usize, y: u64) {
        while y >= self.get_height() {
            self.rocks.push(vec![false; 7]);
        }

        self.rocks[(y - self.height_offset) as usize][x] = true;
    }

    fn try_move_right(&self, shape: &mut Shape) {
        if shape.x + shape.width >= 7 {
            return;
        }
        for (x, y) in shape.points.iter() {
            let x = x + shape.x + 1;
            if x >= 7 {
                panic!("shape [{}, {}] ({}): {:?}", shape.x, shape.y, shape.width, &shape.points);
            }
            let y = *y as u64 + shape.y;
            if self.is_occupied(x, y) {
                return;
            }
        }
        shape.x += 1;
    }

    fn try_move_left(&self, shape: &mut Shape) {
        if shape.x == 0 {
            return;
        }
        for (x, y) in shape.points.iter() {
            let x = x + shape.x - 1;
            let y = *y as u64 + shape.y;
            if self.is_occupied(x, y) {
                return;
            }
        }
        shape.x -= 1;
    }

    fn is_occupied(&self, x: usize, y: u64) -> bool {
        match self.rocks.get((y - self.height_offset) as usize) {
            Some(row) => {
                return *row.get(x).unwrap();
            },
            None => false,
        }
    }

    fn get_height(&self) -> u64 {
        self.rocks.len() as u64 + self.height_offset
    }

    fn new() -> Map {
        Map {
            rocks: Vec::new(),
            height_offset: 0,
        }
    }
}

#[derive(Clone)]
struct Shape {
    // Bottom left of a shape
    x: usize,
    y: u64,
    
    width: usize,
    points: Vec<(usize, usize)>,
}

fn get_shapes() -> Vec<Shape> {
    vec![
        Shape {
            y: 3,
            x: 2,
            width: 4,
            points: vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
            ],
        },
        Shape {
            y: 3,
            x: 2,
            width: 3,
            points: vec![
                (1, 0),
                (0, 1),
                (1, 1),
                (2, 1),
                (1, 2),
            ],
        },
        Shape {
            y: 3,
            x: 2,
            width: 3,
            points: vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (2, 1),
                (2, 2),
            ],
        },
        Shape {
            y: 3,
            x: 2,
            width: 1,
            points: vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
            ],
        },
        Shape {
            y: 3,
            x: 2,
            width: 2,
            points: vec![
                (0, 0),
                (0, 1),
                (1, 0),
                (1, 1),
            ],
        },
    ]
}

fn parse_input(input: &str) -> Vec<bool> {
    input.lines().next().unwrap().chars().map(|c| c == '>').collect()
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = fall_rocks(input, 2022);

        assert_eq!(result, 3068);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "3090");
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = fall_rocks(input, 1000000000000);

        assert_eq!(result, 1514285714288);
    }

    #[test]
    fn part2_input() {
        let result = get_solution_part2();

        assert_eq!(result, "1530057803453");
    }
}
