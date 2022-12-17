
pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = fall_rocks(input);
    return result.to_string();
}

fn fall_rocks(input: &str) -> usize {
    let rocks = 2022;
    let directions = parse_input(input);
    let shapes = get_shapes();

    let mut map = Map::new();
    let mut shapes_iter = shapes.iter().cloned().cycle();
    let mut directions_iter = directions.iter().cloned().cycle();

    for _ in 0..rocks {
        let mut shape = shapes_iter.next().unwrap();
        shape.y = map.get_height() + 3;
        loop {
            let move_right = directions_iter.next().unwrap();
            if move_right {
                map.try_move_right(&mut shape);
            }
            else {
                map.try_move_left(&mut shape);
            }

            if !map.move_shape_down(&mut shape) {
                break;
            }
        }
    }

    return map.get_height();
}

struct Map {
    rocks: Vec<Vec<bool>>,
}

impl Map {
    fn move_shape_down(&mut self, shape: &mut Shape) -> bool {
        for (x, y) in shape.points.iter() {
            let y = y + shape.y;
            if y == 0 {
                self.add_shape(shape);
                return false;
            }
            
            let x = x + shape.x;
            let y = y - 1;
            if self.is_occupied(x, y) {
                self.add_shape(shape);
                return false;
            }
        }

        shape.y -= 1;
        return true;
    }

    fn add_shape(&mut self, shape: &Shape) {
        for (x, y) in shape.points.iter() {
            let x = x + shape.x;
            let y = y + shape.y;
            self.add_point(x, y);
        }
    }

    fn add_point(&mut self, x: usize, y: usize) {
        while y >= self.rocks.len() {
            self.rocks.push(vec![false; 7]);
        }

        self.rocks[y][x] = true;
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
            let y = y + shape.y;
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
            let y = y + shape.y;
            if self.is_occupied(x, y) {
                return;
            }
        }
        shape.x -= 1;
    }

    fn is_occupied(&self, x: usize, y: usize) -> bool {
        match self.rocks.get(y) {
            Some(row) => {
                return *row.get(x).unwrap();
            },
            None => false,
        }
    }

    fn get_height(&self) -> usize {
        self.rocks.len()
    }

    fn new() -> Map {
        Map {
            rocks: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct Shape {
    // Bottom left of a shape
    x: usize,
    y: usize,
    
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
        let result = fall_rocks(input);

        assert_eq!(result, 3068);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "3090");
    }
}
