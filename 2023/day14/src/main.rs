use std::collections::HashMap;

fn main() {
    let input = get_input();
    let map = parse_input(input);
    let result_part1 = map.calculate_weight();
    println!("Part1: {}", result_part1);
}

#[derive(Debug)]
struct Map {
    columns: Vec<Column>,
    rows: usize,
}

impl Map {
    fn calculate_weight(&self) -> usize {
        let mut weight = 0;
        for row in 0..self.rows {
            let rocks = self.columns.iter().map(|col| col.is_rock(row)).filter(|&b| b).count();
            weight += (self.rows - row) * rocks;
        }
        return  weight;
    }

    fn process_input_row(&mut self, input: &str) {
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

#[derive(Debug, Clone)]
struct Column {
    tiles: HashMap<usize, Tile>,
    max: usize,
}

impl Column {
    fn is_rock(&self, row: usize) -> bool {
        if let Some(tile) = self.tiles.get(&row) {
            return tile == &Tile::Rock;
        }
        return false;
    }

    fn process_input(&mut self, char: char, row: usize) {
        match char {
            '.' => {},
            '#' => {
                self.tiles.insert(row, Tile::Column);
                self.max = row + 1;
            },
            'O' => {
                self.tiles.insert(self.max, Tile::Rock);
                self.max += 1;
            },
            _ => panic!(),
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
    Column,
    Rock,
}

fn parse_input(input: &str) -> Map {
    let mut input = input.lines();
    let first_row = input.next().unwrap();
    let mut map = Map::init(first_row.len());
    map.process_input_row(first_row);
    input.for_each(|line| map.process_input_row(line));
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
        let map = parse_input(input);
        let result = map.calculate_weight();
        assert_eq!(result, 136);
    }
}
