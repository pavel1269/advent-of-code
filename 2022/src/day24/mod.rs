use std::collections::{HashMap, HashSet};

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = navigate_blizzard(input);
    return result.to_string();
}

fn navigate_blizzard(input: &str) -> usize {
    let mut valley = Valley::from(input);
    let mut positions = HashSet::from([valley.start.clone()]);
    let mut minutes = 0;
    let end = valley.end.clone().move_up(&valley, false).unwrap();
    loop {
        valley.tick();
        minutes += 1;

        let mut positions_new = HashSet::with_capacity(positions.len() * 2);
        for position in positions.iter().cloned() {
            if position == end {
                return minutes;
            }

            let position_up = position.clone().move_up(&valley, false);
            let position_down = position.clone().move_down(&valley, false);
            let position_left = position.clone().move_left(&valley, false);
            let position_right = position.clone().move_right(&valley, false);

            if valley.is_free(&position) {
                positions_new.insert(position);
            }
            if position_up.is_some() && valley.is_free(&position_up.unwrap()) {
                positions_new.insert(position_up.unwrap());
            }
            if position_down.is_some() && valley.is_free(&position_down.unwrap()) {
                positions_new.insert(position_down.unwrap());
            }
            if position_left.is_some() && valley.is_free(&position_left.unwrap()) {
                positions_new.insert(position_left.unwrap());
            }
            if position_right.is_some() && valley.is_free(&position_right.unwrap()) {
                positions_new.insert(position_right.unwrap());
            }
        }
        positions = positions_new;
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_dir_overlap(&mut self, dir: Direction, valley: &Valley) -> Position {
        match dir {
            Direction::Up => self.move_up(valley, true).unwrap(),
            Direction::Down => self.move_down(valley, true).unwrap(),
            Direction::Left => self.move_left(valley, true).unwrap(),
            Direction::Right => self.move_right(valley, true).unwrap(),
        }
    }

    fn move_up(&mut self, valley: &Valley, overlap: bool) -> Option<Position> {
        if self.y == valley.top {
            return None;
        }
        self.y -= 1;
        if self.y == valley.top {
            if overlap {
                self.y = valley.bottom - 1;
            } else {
                return None;
            }
        }
        return Some(*self);
    }

    fn move_down(&mut self, valley: &Valley, overlap: bool) -> Option<Position> {
        self.y += 1;
        if self.y == valley.bottom {
            if overlap {
                self.y = valley.top + 1;
            } else {
                return None;
            }
        }
        return Some(*self);
    }

    fn move_left(&mut self, valley: &Valley, overlap: bool) -> Option<Position> {
        if self.y == valley.top {
            return None;
        }
        self.x -= 1;
        if self.x == valley.left {
            if overlap {
                self.x = valley.right - 1;
            } else {
                return None;
            }
        }
        return Some(*self);
    }

    fn move_right(&mut self, valley: &Valley, overlap: bool) -> Option<Position> {
        if self.y == valley.top {
            return None;
        }
        self.x += 1;
        if self.x == valley.right {
            if overlap {
                self.x = valley.left + 1;
            } else {
                return None;
            }
        }
        return Some(*self);
    }

    fn from(x: usize, y: usize) -> Position {
        Position { x: x, y: y }
    }
}

#[derive(Debug)]
struct Valley {
    start: Position,
    end: Position,

    top: usize,
    bottom: usize,
    left: usize,
    right: usize,

    blizzards: HashMap<usize, Vec<(usize, Direction)>>,
}

impl Valley {
    fn tick(&mut self) {
        let mut blizzards_new = HashMap::with_capacity(self.blizzards.len());
        for key in self.top + 1..self.bottom {
            blizzards_new.insert(key, Vec::new());
        }
        for index_row in self.blizzards.keys().cloned() {
            let blizzards_row = self.blizzards.get(&index_row).unwrap();
            for (index_column, dir) in blizzards_row.iter().cloned() {
                let position =
                    Position::from(index_column, index_row).move_dir_overlap(dir, self);
                blizzards_new
                    .get_mut(&position.y)
                    .unwrap()
                    .push((position.x, dir));
            }
        }
        self.blizzards = blizzards_new;
    }

    fn is_free(&self, position: &Position) -> bool {
        match self.blizzards.get(&position.y) {
            Some(row) => {
                row.iter().all(|(blizz_x, _)| blizz_x != &position.x)
            },
            None => true,
        }
    }

    fn from(input: &str) -> Valley {
        let height = input.lines().count() - 1;
        let mut lines_iter = input.lines().rev();
        let last_line = lines_iter.next().unwrap();
        let mut lines_iter = lines_iter.rev();
        let first_line = lines_iter.next().unwrap();

        let width = first_line.len() - 1;
        let start = first_line.chars().position(|char| char == '.').unwrap();
        let end = last_line.chars().position(|char| char == '.').unwrap();

        let mut blizzards = HashMap::new();
        for (index_row, row) in lines_iter.enumerate() {
            let mut blizzards_row = Vec::new();
            for (index_column, char) in row.chars().skip(1).take(width - 1).enumerate() {
                let direction = match char {
                    '.' => continue,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!(),
                };
                blizzards_row.push((index_column + 1, direction));
            }
            blizzards.insert(index_row + 1, blizzards_row);
        }

        return Valley {
            start: Position::from(start, 0),
            end: Position::from(end, height),
            top: 0,
            bottom: height,
            left: 0,
            right: width,
            blizzards: blizzards,
        };
    }
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = navigate_blizzard(input);

        assert_eq!(result, 18);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "274");
    }
}
