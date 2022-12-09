use std::collections::HashSet;


pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = calculate_visited_places(input);
    return result.to_string();
}

fn calculate_visited_places(input: &str) -> usize {
    let moves = move_rope(input);
    return moves.len();
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    fn move_by_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Right => self.move_right(),
            Direction::Left => self.move_left(),
        }
    }

    fn move_up(&mut self) {
        self.y += 1;
    }
    fn move_down(&mut self) {
        self.y -= 1;
    }
    fn move_right(&mut self) {
        self.x += 1;
    }
    fn move_left(&mut self) {
        self.x -= 1;
    }
    
    fn move_up_right(&mut self) {
        self.move_up();
        self.move_right();
    }
    fn move_up_left(&mut self) {
        self.move_up();
        self.move_left();
    }
    fn move_down_right(&mut self) {
        self.move_down();
        self.move_right();
    }
    fn move_down_left(&mut self) {
        self.move_down();
        self.move_left();
    }

    fn is_touching(&self, other: &Position) -> bool {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        return x_diff <= 1 && y_diff <= 1;
    }

    fn move_toward(&mut self, target: &Position) {
        let x_diff = target.x - self.x;
        let y_diff = target.y - self.y;

        // target.x 25 - self.x 15 = right 10
        // target.x -10 - self.x -15 = right 5
        let is_right = x_diff > 0;
        let is_left = x_diff < 0;

        // target.y 25 - self.y 15 = above 10
        // target.y -10 - self.y -15 = above 5
        let is_above = y_diff > 0;
        let is_below = y_diff < 0;

        if is_above {
            if is_right {
                self.move_up_right();
            }
            else if is_left {
                self.move_up_left();
            }
            else {
                self.move_up();
            }
        }
        else if is_below {
            if is_right {
                self.move_down_right();
            }
            else if is_left {
                self.move_down_left();
            }
            else {
                self.move_down();
            }
        }
        else {
            if is_right {
                self.move_right();
            }
            else if is_left {
                self.move_left();
            }
            else {
                panic!();
            }
        }
    }
}

fn move_rope(input: &str) -> HashSet<Position> {
    let directions = parse_input(input);
    let mut visited = HashSet::new();
    let mut head = Position::new();
    let mut tail = Position::new();
    visited.insert(tail);

    for direction in directions.iter() {
        for _ in 0..direction.step {
            head.move_by_direction(direction.direction);
            if head.is_touching(&tail) {
                continue;
            }
            
            tail.move_toward(&head);
            visited.insert(tail);
        }
    }
    return visited;
}

struct Move {
    direction: Direction,
    step: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn from_char(direction_char: String) -> Direction {
        match direction_char.as_str() {
            "U" => Self::Up,
            "D" => Self::Down,
            "R" => Self::Right,
            "L" => Self::Left,
            _ => panic!(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    let regex = regex::Regex::new(r"^([A-Z]) (\d+)$").unwrap();
    let count = input.lines().count();
    let mut moves = Vec::with_capacity(count);
    for line in input.lines() {
        match regex.captures(line) {
            None => panic!(),
            Some(captures) => {
                let direction_char = captures[1].parse::<String>().unwrap();
                moves.push(Move
                {
                    direction: Direction::from_char(direction_char),
                    step: captures[2].parse().unwrap(),
                });
            },
        }
    }
    return moves;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = calculate_visited_places(input);

        assert_eq!(result, 13);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "5695");
    }
}
