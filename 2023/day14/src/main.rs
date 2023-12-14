use std::{
    collections::{HashMap, LinkedList},
    fmt::Display,
};

fn main() {
    let input = get_input();
    let map = parse_input_with_tilt(input);

    let result_part1 = map.calculate_weight();
    println!("Part1: {}", result_part1);

    let map = map.tilt_times(3); // finish one rotation
    let result_part2 = weight_after_cycling(map, 1000000000 - 1);
    println!("Part2: {}", result_part2);
}

fn weight_after_cycling(mut map: Map, mut times: usize) -> usize {
    let cycle_size_min = 3;
    let cycle_size_max = 20;

    // 1. detect cycle
    let weight = map.calculate_weight_rotated();
    let mut cycles = LinkedList::new();
    cycles.push_back(Cycle::init(weight));

    while !cycles.iter().any(|c| c.complete) {
        map = map.cycle(1);
        let weight = map.calculate_weight_rotated();
        times -= 1;

        cycles
            .iter_mut()
            .for_each(|c| c.add(weight, cycle_size_min, cycle_size_max));
        cycles.push_back(Cycle::init(weight))
    }

    // 2. move whole cycles forward
    let cycle = cycles.iter().filter(|c| c.complete).next().unwrap();
    let cycle_size = cycle.weights.len();
    let cycle_times = times / cycle_size;
    times = times - cycle_times * cycle_size;

    // 3. finish cycling to desired amount
    map = map.cycle(times);
    let weight = map.calculate_weight_rotated();
    return weight;
}

#[derive(Debug)]
struct Cycle {
    weights: Vec<usize>,
    repeats: usize,
    complete: bool,
    invalid: bool,
}

impl Cycle {
    fn add(&mut self, weight: usize, min: usize, max: usize) {
        if self.complete || self.invalid {
            return;
        }

        if self.weights.len() >= max {
            self.invalid = true;
        } else if self.weights.len() >= min && self.weights[self.repeats] == weight {
            self.repeats += 1;
            if self.weights.len() == self.repeats {
                self.complete = true;
            }
        } else {
            self.weights.push(weight);
            self.repeats = 0;
        }
    }

    fn init(weight: usize) -> Cycle {
        Cycle {
            weights: vec![weight],
            repeats: 0,
            complete: false,
            invalid: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    columns: Vec<Column>,
    rows: usize,
}

impl Map {
    fn calculate_weight(&self) -> usize {
        let mut weight = 0;
        for row in 0..self.rows {
            let rocks = self
                .columns
                .iter()
                .map(|column| column.is_rock(row))
                .filter(|&bool| bool)
                .count();
            weight += (self.rows - row) * rocks;
        }
        return weight;
    }

    fn calculate_weight_rotated(&self) -> usize {
        let weight = self
            .columns
            .iter()
            .enumerate()
            .map(|(index, column)| {
                let mult = self.rows - index;
                let rocks = column
                    .tiles
                    .values()
                    .filter(|&&tile| tile == Tile::Rock)
                    .count();
                return rocks * mult;
            })
            .sum();
        return weight;
    }

    fn cycle(&self, times: usize) -> Self {
        self.tilt_times(4 * times)
    }

    fn tilt_times(&self, times: usize) -> Self {
        if times == 0 {
            return self.clone();
        }
        let mut map = self.tilt_left();
        for _ in 0..times - 1 {
            map = map.tilt_left();
        }
        return map;
    }

    fn tilt_left(&self) -> Self {
        let mut map = Map::init(self.columns.len());
        for (index_row, column) in self.columns.iter().enumerate() {
            for index_column in 0..self.rows {
                let column_new = map.columns.get_mut(index_column).unwrap();
                let tile = column.get_tile(self.rows - index_column - 1);
                column_new.process_tile(tile, index_row);
            }
        }
        map.rows = self.columns.len();
        return map;
    }

    fn process_input_row_with_tilt(&mut self, input: &str) {
        for (index, char) in input.chars().enumerate() {
            let column = self.columns.get_mut(index).unwrap();
            column.process_input(char, self.rows);
        }
        self.rows += 1;
    }

    fn init(size: usize) -> Self {
        Map {
            columns: vec![Column::new(); size],
            rows: 0,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for column in self.columns.iter() {
                write!(f, "{}", column.get_tile(row))?;
            }
            writeln!(f, "")?;
        }
        return Ok(());
    }
}

#[derive(Debug, Clone)]
struct Column {
    tiles: HashMap<usize, Tile>,
    max: usize,
}

impl Column {
    fn is_rock(&self, row: usize) -> bool {
        self.get_tile(row) == Tile::Rock
    }

    fn get_tile(&self, row: usize) -> Tile {
        if let Some(&tile) = self.tiles.get(&row) {
            return tile;
        }
        return Tile::Nothing;
    }

    fn process_input(&mut self, char: char, row: usize) {
        let tile = Tile::from(char);
        self.process_tile(tile, row);
    }

    fn process_tile(&mut self, tile: Tile, row: usize) {
        match tile {
            Tile::Nothing => {}
            Tile::Column => {
                self.tiles.insert(row, Tile::Column);
                self.max = row + 1;
            }
            Tile::Rock => {
                self.tiles.insert(self.max, Tile::Rock);
                self.max += 1;
            }
        }
    }

    fn new() -> Self {
        Column {
            tiles: HashMap::new(),
            max: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Nothing,
    Column,
    Rock,
}

impl Tile {
    fn from(char: char) -> Self {
        match char {
            '.' => Self::Nothing,
            '#' => Self::Column,
            'O' => Self::Rock,
            _ => panic!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Self::Column => '#',
            Self::Nothing => '.',
            Self::Rock => 'O',
        };
        return write!(f, "{}", char);
    }
}

fn parse_input_with_tilt(input: &str) -> Map {
    let mut input = input.lines();
    let first_row = input.next().unwrap();
    let mut map = Map::init(first_row.len());
    map.process_input_row_with_tilt(first_row);
    input.for_each(|line| map.process_input_row_with_tilt(line));
    return map;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        include_str!("./example.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let map = parse_input_with_tilt(input);
        let result = map.calculate_weight();
        assert_eq!(result, 136);
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let map = parse_input_with_tilt(input).tilt_times(3);
        let result = weight_after_cycling(map, 1000000000 - 1);
        assert_eq!(result, 64);
    }
}
