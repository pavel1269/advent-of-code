use std::collections::{hash_map::Entry, HashMap, HashSet};

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = count_empty_ground(input);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = rounds_till_stable(input);
    return result.to_string();
}

fn rounds_till_stable(input: &str) -> usize {
    let mut map = Map::from(input);
    return map.spread_till_stable();
}

fn count_empty_ground(input: &str) -> usize {
    let mut map = Map::from(input);
    map.spread(10);
    return map.calculate_empty();
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North = 0,
    South = 1,
    West = 2,
    East = 3,

    Count = 4,
}

struct Map {
    elves: HashMap<i32, HashSet<i32>>,
}

impl Map {
    fn spread(&mut self, times: usize) {
        let directions = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        for iter in 0..times {
            let order: Vec<Direction> = directions
                .iter()
                .cycle()
                .skip(iter)
                .take(4)
                .cloned()
                .collect();
            self.spread_once(&order);
        }
    }

    fn spread_till_stable(&mut self) -> usize {
        let directions = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        let mut moves = 0;
        loop {
            println!("{:?}", self.elves);
            let order: Vec<Direction> = directions
                .iter()
                .cycle()
                .skip(moves)
                .take(4)
                .cloned()
                .collect();
            moves += 1;
            if !self.spread_once(&order) {
                return moves;
            }
        }
    }

    fn spread_once(&mut self, order: &Vec<Direction>) -> bool {
        let mut moves = Vec::new();
        for y in self.elves.keys().cloned() {
            let row = self.elves.get(&y).unwrap();
            for x in row.iter().cloned() {
                let surrounding = self.get_surrounding(x, y);
                if surrounding.iter().all(|s| !*s) {
                    // Noone around
                    continue;
                }
                for dir in order.iter().cloned() {
                    if surrounding[dir as usize] {
                        continue;
                    }

                    match dir {
                        Direction::North => moves.push(((x, y), (x, y - 1))),
                        Direction::South => moves.push(((x, y), (x, y + 1))),
                        Direction::West => moves.push(((x, y), (x - 1, y))),
                        Direction::East => moves.push(((x, y), (x + 1, y))),
                        _ => panic!(),
                    }
                    break;
                }
            }
        }

        // Filter out duplicates
        let mut uniques = HashSet::new();
        let duplicates: HashSet<(i32, i32)> = moves
            .iter()
            .map(|(_, to)| to)
            .filter(|to| !uniques.insert(*to))
            .cloned()
            .collect();
        moves = moves
            .iter()
            .filter(|(_, to)| !duplicates.contains(to))
            .cloned()
            .collect();

        // Execute moves
        for ((from_x, from_y), (to_x, to_y)) in moves.iter().cloned() {
            if !self.elves.get_mut(&from_y).unwrap().remove(&from_x) {
                panic!("[{}, {}] is empty", from_x, from_y);
            }

            match self.elves.entry(to_y) {
                Entry::Vacant(entry) => {
                    entry.insert(HashSet::from([to_x]));
                }
                Entry::Occupied(mut entry) => {
                    if !entry.get_mut().insert(to_x) {
                        panic!("[{}, {}] -> [{}, {}] occupied", from_x, from_y, to_x, to_y);
                    }
                }
            }
        }

        return moves.len() > 0;
    }

    fn get_surrounding(&self, x: i32, y: i32) -> [bool; Direction::Count as usize] {
        let mut next_nw = false;
        let mut next_n = false;
        let mut next_ne = false;

        let mut next_w = false;
        let mut next_e = false;

        let mut next_sw = false;
        let mut next_s = false;
        let mut next_se = false;

        let row_above = self.elves.get(&(y - 1));
        match row_above {
            Some(row_above) => {
                next_nw = row_above.contains(&(x - 1));
                next_n = row_above.contains(&x);
                next_ne = row_above.contains(&(x + 1));
            }
            None => {}
        }

        let row = self.elves.get(&y);
        match row {
            Some(row) => {
                next_w = row.contains(&(x - 1));
                next_e = row.contains(&(x + 1));
            }
            None => {}
        }

        let row_below = self.elves.get(&(y + 1));
        match row_below {
            Some(row_below) => {
                next_sw = row_below.contains(&(x - 1));
                next_s = row_below.contains(&x);
                next_se = row_below.contains(&(x + 1));
            }
            None => {}
        }

        return [
            next_nw || next_n || next_ne,
            next_sw || next_s || next_se,
            next_nw || next_w || next_sw,
            next_ne || next_e || next_se,
        ];
    }

    fn calculate_empty(&self) -> usize {
        let top = self
            .elves
            .keys()
            .cloned()
            .map(|key| (key, self.elves[&key].len()))
            .filter(|(_, size)| size > &0)
            .map(|(key, _)| key)
            .min_by(|a, b| a.cmp(b))
            .unwrap();
        let bottom = self
            .elves
            .keys()
            .cloned()
            .map(|key| (key, self.elves[&key].len()))
            .filter(|(_, size)| size > &0)
            .map(|(key, _)| key)
            .max_by(|a, b| a.cmp(b))
            .unwrap();

        let left = self
            .elves
            .keys()
            .map(|key| self.elves[key].iter().min())
            .filter(|min| min.is_some())
            .map(|min| min.unwrap())
            .cloned()
            .min()
            .unwrap();
        let right = self
            .elves
            .keys()
            .map(|key| self.elves[key].iter().max())
            .filter(|max| max.is_some())
            .map(|max| max.unwrap())
            .cloned()
            .max()
            .unwrap();

        let mut empty = 0;
        for index_row in top..bottom + 1 {
            match self.elves.get(&index_row) {
                None => {}
                Some(row) => {
                    let elves = (left..right + 1)
                        .map(|index_column| row.contains(&index_column))
                        .filter(|elf| !*elf)
                        .count();
                    empty += elves;
                }
            }
        }
        return empty;
    }

    fn from(text: &str) -> Map {
        let mut elves = HashMap::new();
        for (index_row, row) in text.lines().enumerate() {
            let mut elves_row = HashSet::new();
            for (index_column, char) in row.chars().enumerate() {
                let index_column = index_column as i32;
                match char {
                    '.' => continue,
                    '#' => {
                        elves_row.insert(index_column);
                    }
                    _ => panic!(),
                }
            }

            let index_row = index_row as i32;
            elves.insert(index_row, elves_row);
        }

        let map = Map { elves };
        return map;
    }
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    fn get_example_input1() -> &'static str {
        ".....
..##.
..#..
.....
..##.
....."
    }

    fn get_example_input2() -> &'static str {
        "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
    }

    #[test]
    fn map_parse_count_example1() {
        let input = get_example_input1();
        let map = Map::from(input);

        let result = map.calculate_empty();

        assert_eq!(result, 3);
    }

    #[test]
    fn map_parse_count_example2() {
        let input = get_example_input2();
        let map = Map::from(input);

        let result = map.calculate_empty();

        assert_eq!(result, 27);
    }

    #[test_case(0, 0, false, false, false, false; "0,0")]
    #[test_case(1, 1, false, true, false, true; "1,1")]
    #[test_case(3, 0, false, true, true, false; "3,0")]
    #[test_case(0, 3, false, false, false, false; "0,3")]
    #[test_case(13, 0, false, false, false, false; "13,0")]
    #[test_case(0, 13, false, false, false, false; "0,13")]
    #[test_case(-2, 0, false, false, false, false; "-2,0")]
    #[test_case(0, -6, false, false, false, false; "0,-6")]
    fn map_parse_surrounding_example1(x: i32, y: i32, b1: bool, b2: bool, b3: bool, b4: bool) {
        let input = get_example_input1();
        let map = Map::from(input);

        let result = map.get_surrounding(x, y);

        assert_eq!(result, [b1, b2, b3, b4]);
    }

    #[test]
    fn part1_example1() {
        let input = get_example_input1();
        let result = count_empty_ground(input);

        assert_eq!(result, 25);
    }

    #[test]
    fn part1_example2() {
        let input = get_example_input2();
        let result = count_empty_ground(input);

        assert_eq!(result, 110);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "3780");
    }

    #[test]
    fn part2_example1() {
        let input = get_example_input1();
        let result = rounds_till_stable(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn part2_example2() {
        let input = get_example_input2();
        let result = rounds_till_stable(input);

        assert_eq!(result, 20);
    }

    #[test]
    fn part2_input() {
        let result = get_solution_part2();

        assert_eq!(result, "930");
    }
}
