pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = get_password(input);
    return result.to_string();
}

fn get_password(input: &str) -> usize {
    let (map, commands) = parse_input(input);
    let mut map = Map::from(map);
    for command in commands.iter() {
        map.execute(command);
    }

    let password =
        (map.position_y + 1) * 1000 + (map.position_x + 1) * 4 + map.direction.get_value();
    return password;
}

struct Map {
    map: Vec<Vec<MapTile>>,
    height: usize,
    width: usize,

    position_x: usize,
    position_y: usize,
    direction: Direction,
}

impl Map {
    fn execute(&mut self, command: &Command) {
        match command {
            Command::Turn(direction) => self.turn(direction),
            Command::Move(moves) => self.move_by(*moves),
        }
    }

    fn move_by(&mut self, mut moves: usize) {
        while moves > 0 {
            let (x, y, tile) = self.get_next_tile();
            match tile {
                MapTile::None => panic!(),
                MapTile::Wall => break,
                MapTile::Empty => {
                    self.position_x = x;
                    self.position_y = y;
                    moves -= 1;
                }
            }
        }
    }

    fn get_next_tile(&self) -> (usize, usize, MapTile) {
        match self.direction {
            Direction::Up => self.get_tile_above(),
            Direction::Down => self.get_tile_below(),
            Direction::Left => self.get_tile_left(),
            Direction::Right => self.get_tile_right(),
        }
    }

    fn get_next_tile_move(
        &self,
        move_fn: fn(map: &Map, usize, usize) -> (usize, usize),
    ) -> (usize, usize, MapTile) {
        let mut x = self.position_x;
        let mut y = self.position_y;
        loop {
            (x, y) = move_fn(self, x, y);
            let tile = self.map[y][x];
            if tile == MapTile::None {
                continue;
            }

            return (x, y, tile);
        }
    }

    fn get_tile_above(&self) -> (usize, usize, MapTile) {
        self.get_next_tile_move(Self::move_up)
    }

    fn move_up(&self, x: usize, mut y: usize) -> (usize, usize) {
        if y == 0 {
            y = self.height - 1;
        } else {
            y -= 1;
        }
        return (x, y);
    }

    fn get_tile_below(&self) -> (usize, usize, MapTile) {
        self.get_next_tile_move(Self::move_down)
    }

    fn move_down(&self, x: usize, mut y: usize) -> (usize, usize) {
        if y == self.height - 1 {
            y = 0;
        } else {
            y += 1;
        }
        return (x, y);
    }

    fn get_tile_left(&self) -> (usize, usize, MapTile) {
        self.get_next_tile_move(Self::move_left)
    }

    fn move_left(&self, mut x: usize, y: usize) -> (usize, usize) {
        if x == 0 {
            x = self.width - 1;
        } else {
            x -= 1;
        }
        return (x, y);
    }

    fn get_tile_right(&self) -> (usize, usize, MapTile) {
        self.get_next_tile_move(Self::move_right)
    }

    fn move_right(&self, mut x: usize, y: usize) -> (usize, usize) {
        if x == self.width - 1 {
            x = 0;
        } else {
            x += 1;
        }
        return (x, y);
    }

    fn turn(&mut self, direction: &Direction) {
        self.direction = match self.direction {
            Direction::Up => {
                if direction == &Direction::Left {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
            Direction::Down => {
                if direction == &Direction::Left {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            Direction::Left => {
                if direction == &Direction::Left {
                    Direction::Down
                } else {
                    Direction::Up
                }
            }
            Direction::Right => {
                if direction == &Direction::Left {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
        };
    }

    fn from(mut map: Vec<Vec<MapTile>>) -> Map {
        let (x, _) = map[0]
            .iter()
            .enumerate()
            .skip_while(|(_, tile)| tile == &&MapTile::None)
            .next()
            .unwrap();
        let height = map.len();
        let width = map.iter().map(|row| row.len()).max().unwrap();

        for row in map.iter_mut() {
            while row.len() < width {
                row.push(MapTile::None);
            }
        }

        Map {
            map,
            height: height,
            width: width,
            position_x: x,
            position_y: 0,
            direction: Direction::Right,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum MapTile {
    None,
    Empty,
    Wall,
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_value(&self) -> usize {
        match self {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }
}

enum Command {
    Move(usize),
    Turn(Direction),
}

fn parse_input(input: &str) -> (Vec<Vec<MapTile>>, Vec<Command>) {
    let mut lines_iter = input.lines().rev();
    let last_line = lines_iter.next().unwrap();
    lines_iter.next();
    let lines_iter = lines_iter.rev();

    let commands = parse_input_command(last_line);
    let map = parse_input_map(lines_iter);

    return (map, commands);
}

fn parse_input_map<'a>(lines: impl Iterator<Item = &'a str> + Clone) -> Vec<Vec<MapTile>> {
    let height = lines.clone().count();
    let mut map = Vec::with_capacity(height);
    for line in lines {
        let width = line.len();
        let mut row = Vec::with_capacity(width);

        for point in line.chars() {
            let tile = match point {
                ' ' => MapTile::None,
                '.' => MapTile::Empty,
                '#' => MapTile::Wall,
                _ => panic!(),
            };
            row.push(tile);
        }

        map.push(row);
    }
    return map;
}

fn parse_input_command(command: &str) -> Vec<Command> {
    let mut directions = Vec::new();
    for command in command.replace("R", " R ").replace("L", " L ").split(" ") {
        if command == "R" {
            directions.push(Command::Turn(Direction::Right));
        } else if command == "L" {
            directions.push(Command::Turn(Direction::Left));
        } else {
            let moves = command.parse().unwrap();
            directions.push(Command::Move(moves));
        }
    }
    return directions;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = get_password(input);

        assert_eq!(result, 6032);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "181128");
    }
}
