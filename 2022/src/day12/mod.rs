use std::collections::{HashMap, hash_map::{Entry}};

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = find_path_length(input);
    return result.to_string();
}

fn find_path_length(input: &str) -> usize {
    let map = parse_input(input);

    // position, moves, estimate
    let mut nodes: Vec<(Coord, usize, usize)> = Vec::new();
    nodes.push((map.start, 0, estimate_remaining_distance(&map, &map.start)));

    let mut visited: HashMap<Coord, usize> = HashMap::new();
    while let Some((position, moves, _)) = nodes.pop() {
        // println!("Now at [{}, {}] ({}), target: [{}, {}]", position.x, position.y, map.get_heigh(&position), map.end.x, map.end.y);
        
        if position == map.end {
            // print_debug(&map, &visited);
            return moves;
        }

        match visited.entry(position) {
            Entry::Occupied(mut entry) => {
                if entry.get() <= &moves {
                    continue;
                }
                entry.insert(moves);
            },
            Entry::Vacant(entry) => {
                entry.insert(moves);
            },
        }

        for next_move in get_possible_moves(&map, &position).iter().copied() {
            nodes.push((next_move, moves + 1, estimate_remaining_distance(&map, &next_move)));
        }
        nodes.sort_by(|a, b| b.1.cmp(&a.1));
    }

    print_debug(&map, &visited);
    panic!();
}

fn print_debug(map: &Map, visited: &HashMap<Coord, usize>) {
    println!("Visited:");
    for row_index in 0..map.height {
        for column_index in 0..map.width {
            let pos = Coord::from(column_index, row_index);
            if visited.contains_key(&pos) {
                let moves = visited[&pos];
                print!("{:#03} ", moves);
            }
            else {
                print!(" .  ");
            }
            print!("{} ", char::from_u32(map.get_heigh(&pos) as u32 + 'a' as u32).unwrap());
        }
        println!();
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    points: Vec<Vec<usize>>,
    start: Coord,
    end: Coord,
}

impl Map {
    fn get_heigh(&self, position: &Coord) -> usize {
        self.points[position.y][position.x]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn from(x: usize, y: usize) -> Coord {
        Coord
        {
            x: x,
            y: y,
        }
    }

    fn move_up(&self) -> Option<Coord> {
        if self.y == 0 {
            return None;
        }
        return Some(Coord::from(self.x, self.y - 1));
    }

    fn move_down(&self, map: &Map) -> Option<Coord> {
        if self.y + 1 >= map.height {
            return None;
        }
        return Some(Coord::from(self.x, self.y + 1));
    }

    fn move_left(&self) -> Option<Coord> {
        if self.x == 0 {
            return None;
        }
        return Some(Coord::from(self.x - 1, self.y));
    }

    fn move_right(&self, map: &Map) -> Option<Coord> {
        if self.x + 1 >= map.width {
            return None;
        }
        return Some(Coord::from(self.x + 1, self.y));
    }
}

fn get_possible_moves(map: &Map, position: &Coord) -> Vec<Coord> {
    let pos_height = map.get_heigh(position);
    let mut moves = Vec::new();

    if let Some(pos) = position.move_up() {
        let height = map.get_heigh(&pos);
        if height <= pos_height + 1 {
            moves.push(pos);
        }
    }

    if let Some(pos) = position.move_down(map) {
        let height = map.get_heigh(&pos);
        if height <= pos_height + 1 {
            moves.push(pos);
        }
    }

    if let Some(pos) = position.move_left() {
        let height = map.get_heigh(&pos);
        if height <= pos_height + 1 {
            moves.push(pos);
        }
    }

    if let Some(pos) = position.move_right(map) {
        let height = map.get_heigh(&pos);
        if height <= pos_height + 1 {
            moves.push(pos);
        }
    }

    return moves;
}

fn estimate_remaining_distance(map: &Map, position: &Coord) -> usize {
    let x_diff = position.x.abs_diff(map.end.x);
    let y_diff = position.y.abs_diff(map.end.y);
    let height_diff = 'z' as usize - map.points[position.y][position.x];

    return x_diff + y_diff + height_diff;
}

fn parse_input(input: &str) -> Map {
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();

    let mut map: Vec<Vec<usize>> = Vec::with_capacity(height);
    let mut start = None;
    let mut end = None;

    let base = 'a' as usize;
    for (row_index, line) in input.lines().enumerate() {
        let mut map_row = Vec::with_capacity(width);

        for (column_index, char) in line.chars().enumerate() {
            let point_height: usize;
            if char == 'S' {
                if let Some(_) = start {
                    panic!();
                }
                start = Some(Coord::from(column_index, row_index));
                point_height = 0;
            }
            else if char == 'E' {
                if let Some(_) = end {
                    panic!();
                }
                end = Some(Coord::from(column_index, row_index));
                point_height = 'z' as usize - base;
            }
            else
            {
                point_height = char as usize - base;
            }

            map_row.push(point_height);
        }

        map.push(map_row);
    }

    let map = Map
    {
        width: width,
        height: height,
        points: map,
        start: start.unwrap(),
        end: end.unwrap(),
    };

    return map;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = find_path_length(input);

        assert_eq!(result, 31);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "383");
    }
}
