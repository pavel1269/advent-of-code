use std::mem::swap;

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = get_password_from_map(input);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = get_password_from_cube(input, false);
    return result.to_string();
}

fn get_password_from_cube(input: &str, is_example: bool) -> usize {
    let (map, commands) = parse_input(input);
    let mut cube = if is_example {
        Cube::from_example(map)
    } else {
        Cube::from_input(map)
    };
    for command in commands.iter() {
        cube.execute(command);
    }
    return cube.get_password(is_example);
}

struct Cube {
    facets: [Vec<Vec<MapTile>>; 6],
    dimension: usize,

    facelet: usize,
    x: usize,
    y: usize,
    direction: Direction,
}

impl Cube {
    fn get_password(&self, is_example: bool) -> usize {
        let x;
        let y;
        if is_example {
            match self.facelet {
                1 => {
                    x = self.x + 2 * self.dimension;
                    y = self.y;
                }
                2 => {
                    x = self.x;
                    y = self.y + self.dimension;
                }
                3 => {
                    x = self.x + self.dimension;
                    y = self.y + self.dimension;
                }
                4 => {
                    x = self.x + 2 * self.dimension;
                    y = self.y + self.dimension;
                }
                5 => {
                    x = self.x + 2 * self.dimension;
                    y = self.y + 2 * self.dimension;
                }
                6 => {
                    x = self.x + 3 * self.dimension;
                    y = self.y + 2 * self.dimension;
                }
                _ => panic!(),
            }
        } else {
            match self.facelet {
                1 => {
                    x = self.x + self.dimension;
                    y = self.y;
                }
                2 => {
                    x = self.invert(self.y);
                    y = self.x + 3 * self.dimension;
                }
                3 => {
                    x = self.invert(self.y);
                    y = self.x + 2 * self.dimension;
                }
                4 => {
                    x = self.x + self.dimension;
                    y = self.y + self.dimension;
                }
                5 => {
                    x = self.x + self.dimension;
                    y = self.y + 2 * self.dimension;
                }
                6 => {
                    x = self.invert(self.y) + 2 * self.dimension;
                    y = self.invert(self.x);
                }
                _ => panic!(),
            }
        }

        let password = (y + 1) * 1000 + (x + 1) * 4 + self.direction.get_value();
        return password;
    }

    fn execute(&mut self, command: &Command) {
        match command {
            Command::Turn(direction) => self.turn(direction),
            Command::Move(moves) => self.move_by(*moves),
        }
    }

    fn move_by(&mut self, mut moves: usize) {
        while moves > 0 {
            let (direction, facelet, x, y, tile) = self.get_next_tile();
            match tile {
                MapTile::None => panic!(),
                MapTile::Wall => break,
                MapTile::Empty => {
                    self.direction = direction;
                    self.facelet = facelet;
                    self.x = x;
                    self.y = y;
                    moves -= 1;
                }
            }
        }
    }

    fn get_next_tile(&self) -> (Direction, usize, usize, usize, MapTile) {
        match self.direction {
            Direction::Up => self.get_tile_above(),
            Direction::Down => self.get_tile_below(),
            Direction::Left => self.get_tile_left(),
            Direction::Right => self.get_tile_right(),
        }
    }

    fn get_next_tile_move(
        &self,
        move_fn: fn(map: &Cube, Direction, usize, usize, usize) -> (Direction, usize, usize, usize),
    ) -> (Direction, usize, usize, usize, MapTile) {
        let mut direction = self.direction;
        let mut facelet = self.facelet;
        let mut x = self.x;
        let mut y = self.y;
        loop {
            (direction, facelet, x, y) = move_fn(self, direction, facelet, x, y);
            let tile = self.facets[facelet - 1][y][x];
            if tile == MapTile::None {
                panic!();
            }

            return (direction, facelet, x, y, tile);
        }
    }

    fn get_tile_above(&self) -> (Direction, usize, usize, usize, MapTile) {
        self.get_next_tile_move(Self::move_up)
    }

    fn move_up(
        &self,
        mut direction: Direction,
        mut facelet: usize,
        mut x: usize,
        mut y: usize,
    ) -> (Direction, usize, usize, usize) {
        if y == 0 {
            match facelet {
                1 => {
                    direction = Direction::Down;
                    facelet = 2;
                    x = self.invert(x);
                }
                2 => {
                    direction = Direction::Down;
                    facelet = 1;
                    x = self.invert(x);
                }
                3 => {
                    direction = Direction::Right;
                    facelet = 1;
                    swap(&mut x, &mut y);
                }
                4 => {
                    facelet = 1;
                    y = self.max_coord();
                }
                5 => {
                    facelet = 4;
                    y = self.max_coord();
                }
                6 => {
                    facelet = 4;
                    direction = Direction::Left;
                    y = self.invert(x);
                    x = self.max_coord();
                }
                _ => panic!(),
            }
        } else {
            y -= 1;
        }
        return (direction, facelet, x, y);
    }

    fn get_tile_below(&self) -> (Direction, usize, usize, usize, MapTile) {
        self.get_next_tile_move(Self::move_down)
    }

    fn move_down(
        &self,
        mut direction: Direction,
        mut facelet: usize,
        mut x: usize,
        mut y: usize,
    ) -> (Direction, usize, usize, usize) {
        if y == self.max_coord() {
            match facelet {
                1 => {
                    facelet = 4;
                    y = 0;
                }
                2 => {
                    direction = Direction::Up;
                    facelet = 5;
                    x = self.invert(x);
                }
                3 => {
                    direction = Direction::Right;
                    facelet = 5;
                    y = self.invert(x);
                    x = 0;
                }
                4 => {
                    facelet = 5;
                    y = 0;
                }
                5 => {
                    direction = Direction::Up;
                    facelet = 2;
                    x = self.invert(x);
                }
                6 => {
                    direction = Direction::Right;
                    facelet = 2;
                    y = self.invert(x);
                    x = 0;
                }
                _ => panic!(),
            }
        } else {
            y += 1;
        }
        return (direction, facelet, x, y);
    }

    fn get_tile_left(&self) -> (Direction, usize, usize, usize, MapTile) {
        self.get_next_tile_move(Self::move_left)
    }

    fn move_left(
        &self,
        mut direction: Direction,
        mut facelet: usize,
        mut x: usize,
        mut y: usize,
    ) -> (Direction, usize, usize, usize) {
        if x == 0 {
            match facelet {
                1 => {
                    direction = Direction::Down;
                    facelet = 3;
                    x = y;
                    y = 0;
                }
                2 => {
                    direction = Direction::Up;
                    facelet = 6;
                    x = self.invert(y);
                    y = self.max_coord();
                }
                3 => {
                    facelet = 2;
                    x = self.max_coord();
                }
                4 => {
                    facelet = 3;
                    x = self.max_coord();
                }
                5 => {
                    direction = Direction::Up;
                    facelet = 3;
                    x = self.invert(y);
                    y = self.max_coord();
                }
                6 => {
                    facelet = 5;
                    x = self.max_coord();
                }
                _ => panic!(),
            }
        } else {
            x -= 1;
        }
        return (direction, facelet, x, y);
    }

    fn get_tile_right(&self) -> (Direction, usize, usize, usize, MapTile) {
        self.get_next_tile_move(Self::move_right)
    }

    fn move_right(
        &self,
        mut direction: Direction,
        mut facelet: usize,
        mut x: usize,
        mut y: usize,
    ) -> (Direction, usize, usize, usize) {
        if x == self.max_coord() {
            match facelet {
                1 => {
                    direction = Direction::Left;
                    facelet = 6;
                    y = self.invert(y);
                }
                2 => {
                    facelet = 3;
                    x = 0;
                }
                3 => {
                    facelet = 4;
                    x = 0;
                }
                4 => {
                    direction = Direction::Down;
                    facelet = 6;
                    x = self.invert(y);
                    y = 0;
                }
                5 => {
                    facelet = 6;
                    x = 0;
                }
                6 => {
                    direction = Direction::Left;
                    facelet = 1;
                    y = self.invert(y);
                }
                _ => panic!(),
            }
        } else {
            x += 1;
        }
        return (direction, facelet, x, y);
    }

    fn invert(&self, coordinate: usize) -> usize {
        self.max_coord() - coordinate
    }

    fn max_coord(&self) -> usize {
        self.dimension - 1
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

    fn from_input(map: Vec<Vec<MapTile>>) -> Cube {
        let dimension = map.len() / 4;
        let mut facelet_1 = Vec::with_capacity(dimension);
        let mut facelet_2 = Vec::with_capacity(dimension);
        for row in map.iter().take(dimension) {
            let row_1 = row
                .iter()
                .cloned()
                .skip(dimension)
                .take(dimension)
                .collect();
            facelet_1.push(row_1);
            let row_2 = row
                .iter()
                .cloned()
                .skip(2 * dimension)
                .take(dimension)
                .collect();
            facelet_2.push(row_2);
        }

        let mut facelet_3 = Vec::with_capacity(dimension);
        for row in map.iter().skip(dimension).take(dimension) {
            let row_3 = row
                .iter()
                .cloned()
                .skip(dimension)
                .take(dimension)
                .collect();
            facelet_3.push(row_3);
        }

        let mut facelet_4 = Vec::with_capacity(dimension);
        let mut facelet_5 = Vec::with_capacity(dimension);
        for row in map.iter().skip(2 * dimension).take(dimension) {
            let row_4 = row.iter().cloned().take(dimension).collect();
            facelet_4.push(row_4);
            let row_5 = row
                .iter()
                .cloned()
                .skip(dimension)
                .take(dimension)
                .collect();
            facelet_5.push(row_5);
        }

        let mut facelet_6 = Vec::with_capacity(dimension);
        for row in map.iter().skip(3 * dimension).take(dimension) {
            let row_6 = row.iter().cloned().take(dimension).collect();
            facelet_6.push(row_6);
        }

        facelet_4 = Self::rotate_facelet_right(facelet_4);
        facelet_6 = Self::rotate_facelet_right(facelet_6);
        facelet_2 = Self::rotate_facelet_upside(facelet_2);

        return Cube {
            facets: [
                facelet_1, facelet_6, facelet_4, facelet_3, facelet_5, facelet_2,
            ],
            dimension,
            facelet: 1,
            x: 0,
            y: 0,
            direction: Direction::Right,
        };
    }

    fn rotate_facelet_right(facelet: Vec<Vec<MapTile>>) -> Vec<Vec<MapTile>> {
        let mut result: Vec<Vec<MapTile>> = vec![Vec::with_capacity(facelet.len()); facelet.len()];
        for row in facelet.iter().rev() {
            for (index, tile) in row.iter().cloned().enumerate() {
                result[index].push(tile);
            }
        }
        return result;
    }

    fn rotate_facelet_upside(facelet: Vec<Vec<MapTile>>) -> Vec<Vec<MapTile>> {
        let mut result = Vec::with_capacity(facelet.len());
        for row in facelet.iter().rev() {
            let row = row.iter().cloned().rev().collect();
            result.push(row);
        }
        return result;
    }

    fn from_example(map: Vec<Vec<MapTile>>) -> Cube {
        let dimension = map.len() / 3;
        let mut facelet_1 = Vec::with_capacity(dimension);
        for row in map.iter().take(dimension) {
            let row = row.iter().cloned().skip(dimension * 2).collect();
            facelet_1.push(row);
        }

        let mut facelet_2 = Vec::with_capacity(dimension);
        let mut facelet_3 = Vec::with_capacity(dimension);
        let mut facelet_4 = Vec::with_capacity(dimension);
        for row in map.iter().skip(dimension).take(dimension) {
            let row_2 = row.iter().cloned().take(dimension).collect();
            facelet_2.push(row_2);
            let row_3 = row
                .iter()
                .cloned()
                .skip(dimension)
                .take(dimension)
                .collect();
            facelet_3.push(row_3);
            let row_4 = row
                .iter()
                .cloned()
                .skip(2 * dimension)
                .take(dimension)
                .collect();
            facelet_4.push(row_4);
        }

        let mut facelet_5 = Vec::with_capacity(dimension);
        let mut facelet_6 = Vec::with_capacity(dimension);
        for row in map.iter().skip(2 * dimension).take(dimension) {
            let row_5 = row
                .iter()
                .cloned()
                .skip(2 * dimension)
                .take(dimension)
                .collect();
            facelet_5.push(row_5);
            let row_6 = row
                .iter()
                .cloned()
                .skip(3 * dimension)
                .take(dimension)
                .collect();
            facelet_6.push(row_6);
        }

        return Cube {
            facets: [
                facelet_1, facelet_2, facelet_3, facelet_4, facelet_5, facelet_6,
            ],
            dimension,
            facelet: 1,
            x: 0,
            y: 0,
            direction: Direction::Right,
        };
    }
}

fn get_password_from_map(input: &str) -> usize {
    let (map, commands) = parse_input(input);
    let mut map = Map::from(map);
    for command in commands.iter() {
        map.execute(command);
    }

    let password = (map.y + 1) * 1000 + (map.x + 1) * 4 + map.direction.get_value();
    return password;
}

struct Map {
    map: Vec<Vec<MapTile>>,
    height: usize,
    width: usize,

    x: usize,
    y: usize,
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
                    self.x = x;
                    self.y = y;
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
        let mut x = self.x;
        let mut y = self.y;
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
            x,
            y: 0,
            direction: Direction::Right,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum MapTile {
    None,
    Empty,
    Wall,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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
    use test_case::test_case;

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
        let result = get_password_from_map(input);

        assert_eq!(result, 6032);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "181128");
    }

    #[test_case(Direction::Up, 1, 0, 0, Direction::Down, 2, 2, 0; "face 1 [0,0] move up")]
    #[test_case(Direction::Up, 1, 2, 0, Direction::Down, 2, 0, 0; "face 1 [2,0] move up")]
    #[test_case(Direction::Down, 1, 0, 2, Direction::Down, 4, 0, 0; "face 1 [0,2] move down")]
    #[test_case(Direction::Down, 1, 2, 2, Direction::Down, 4, 2, 0; "face 1 [2,2] move down")]
    #[test_case(Direction::Left, 1, 0, 0, Direction::Down, 3, 0, 0; "face 1 [0,0] move left")]
    #[test_case(Direction::Left, 1, 0, 2, Direction::Down, 3, 2, 0; "face 1 [0,2] move left")]
    #[test_case(Direction::Right, 1, 2, 0, Direction::Left, 6, 2, 2; "face 1 [2,0] move right")]
    #[test_case(Direction::Right, 1, 2, 2, Direction::Left, 6, 2, 0; "face 1 [2,2] move right")]
    #[test_case(Direction::Up, 2, 0, 0, Direction::Down, 1, 2, 0; "face 2 [0,0] move up")]
    #[test_case(Direction::Up, 2, 2, 0, Direction::Down, 1, 0, 0; "face 2 [2,0] move up")]
    #[test_case(Direction::Down, 2, 0, 2, Direction::Up, 5, 2, 2; "face 2 [0,2] move down")]
    #[test_case(Direction::Down, 2, 2, 2, Direction::Up, 5, 0, 2; "face 2 [2,2] move down")]
    #[test_case(Direction::Left, 2, 0, 0, Direction::Up, 6, 2, 2; "face 2 [0,0] move left")]
    #[test_case(Direction::Left, 2, 0, 2, Direction::Up, 6, 0, 2; "face 2 [0,2] move left")]
    #[test_case(Direction::Right, 2, 2, 0, Direction::Right, 3, 0, 0; "face 2 [2,0] move right")]
    #[test_case(Direction::Right, 2, 2, 2, Direction::Right, 3, 0, 2; "face 2 [2,2] move right")]
    fn cube_move(
        direction: Direction,
        facelet: usize,
        x: usize,
        y: usize,
        expected_direction: Direction,
        expected_facelet: usize,
        expected_x: usize,
        expected_y: usize,
    ) {
        let mut cube = Cube {
            dimension: 3,
            x: x,
            y: y,
            direction: direction,
            facelet: facelet,
            facets: [
                vec![vec![MapTile::Empty; 3]; 3],
                vec![vec![MapTile::Empty; 3]; 3],
                vec![vec![MapTile::Empty; 3]; 3],
                vec![vec![MapTile::Empty; 3]; 3],
                vec![vec![MapTile::Empty; 3]; 3],
                vec![vec![MapTile::Empty; 3]; 3],
            ],
        };

        cube.execute(&Command::Move(1));

        assert_eq!(expected_facelet, cube.facelet);
        assert_eq!(expected_direction, cube.direction);
        assert_eq!(expected_x, cube.x);
        assert_eq!(expected_y, cube.y);
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = get_password_from_cube(input, true);

        assert_eq!(result, 5031);
    }

    fn get_example_input2() -> &'static str {
        "     ..#...##..
     .##..#..#.
     #.#....#..
     ..#...#...
     .###.####.
     .###.
     ....#
     ..##.
     ....#
     .###.
..#...###.
.#....##..
#.#.....#.
####.#..#.
..#...##..
.#...
.#...
.###.
.#.#.
.###.

1R1"
    }

    #[test]
    fn part2_parse_input_rotate_facet_right() {
        let facet = vec![
            vec![MapTile::Empty, MapTile::Wall, MapTile::Empty],
            vec![MapTile::Empty, MapTile::Wall, MapTile::Empty],
            vec![MapTile::Wall, MapTile::Wall, MapTile::Wall],
        ];

        let result = Cube::rotate_facelet_right(facet);

        assert_eq!(
            result,
            vec![
                vec![MapTile::Wall, MapTile::Empty, MapTile::Empty],
                vec![MapTile::Wall, MapTile::Wall, MapTile::Wall],
                vec![MapTile::Wall, MapTile::Empty, MapTile::Empty],
            ]
        );
    }

    #[test]
    fn part2_parse_input_rotate_facet_upside() {
        let facet = vec![
            vec![MapTile::Empty, MapTile::Empty, MapTile::Wall],
            vec![MapTile::Empty, MapTile::Empty, MapTile::Wall],
            vec![MapTile::Wall, MapTile::Wall, MapTile::Wall],
        ];

        let result = Cube::rotate_facelet_upside(facet);

        assert_eq!(
            result,
            vec![
                vec![MapTile::Wall, MapTile::Wall, MapTile::Wall],
                vec![MapTile::Wall, MapTile::Empty, MapTile::Empty],
                vec![MapTile::Wall, MapTile::Empty, MapTile::Empty],
            ]
        );
    }

    #[test]
    fn part2_parse_input() {
        let input = get_example_input2();
        let (map, _) = parse_input(input);

        let cube = Cube::from_input(map);

        // facet 1
        assert_eq!(
            cube.facets[0],
            vec![
                vec![
                    MapTile::Empty,
                    MapTile::Empty,
                    MapTile::Wall,
                    MapTile::Empty,
                    MapTile::Empty
                ],
                vec![
                    MapTile::Empty,
                    MapTile::Wall,
                    MapTile::Wall,
                    MapTile::Empty,
                    MapTile::Empty
                ],
                vec![
                    MapTile::Wall,
                    MapTile::Empty,
                    MapTile::Wall,
                    MapTile::Empty,
                    MapTile::Empty
                ],
                vec![
                    MapTile::Empty,
                    MapTile::Empty,
                    MapTile::Wall,
                    MapTile::Empty,
                    MapTile::Empty
                ],
                vec![
                    MapTile::Empty,
                    MapTile::Wall,
                    MapTile::Wall,
                    MapTile::Wall,
                    MapTile::Empty
                ],
            ]
        );

        // facet 2
        assert_eq!(
            cube.facets[1],
            vec![
                vec![
                    MapTile::Empty,
                    MapTile::Empty,
                    MapTile::Empty,
                    MapTile::Empty,
                    MapTile::Empty
                ],
                vec![
                    MapTile::Wall,
                    MapTile::Wall,
                    MapTile::Wall,
                    MapTile::Wall,
                    MapTile::Wall
                ],
                vec![
                    MapTile::Wall,
                    MapTile::Empty,
                    MapTile::Wall,
                    MapTile::Empty,
                    MapTile::Empty
                ],
                vec![
                    MapTile::Wall,
                    MapTile::Wall,
                    MapTile::Wall,
                    MapTile::Empty,
                    MapTile::Empty
                ],
                vec![
                    MapTile::Empty,
                    MapTile::Empty,
                    MapTile::Empty,
                    MapTile::Empty,
                    MapTile::Empty
                ],
            ]
        );

        // assert_eq!(
        //     cube.facets[0][0],
        //     vec![MapTile::Empty, MapTile::Empty, MapTile::Empty, MapTile::Empty, MapTile::Empty]
        // );
    }

    #[test]
    fn part2_input() {
        let result = get_solution_part2();

        assert_eq!(result, "52311");
    }
}
