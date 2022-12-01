use std::collections::HashMap;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = calculate_risk_level(input);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = calculate_largest_basins(input);
    return result;
}

fn calculate_largest_basins(input: &str) -> i64 {
    let map = parse_input(input);
    let mut basin_count: usize = 0;
    // x,y -> basin "id"
    let mut basins: HashMap<(usize, usize), usize> = HashMap::new();
    let mut basin_sizes: Vec<i64> = Vec::new();

    for (row_index, row) in map.iter().enumerate() {
        for (column_index, height) in row.iter().copied().enumerate() {
            if height == 9 {
                continue;
            }

            let basin_key = &(row_index, column_index);
            let basin_up_key = (row_index > 0).then(|| (row_index - 1, column_index));
            let basin_left_key = (column_index > 0).then(|| (row_index, column_index - 1));

            let basin_up_exists = row_index > 0 && basins.contains_key(&basin_up_key.unwrap());
            let basin_left_exists = column_index > 0 && basins.contains_key(&basin_left_key.unwrap());

            if basin_left_exists && basin_up_exists {
                // merge them
                let basin_up = basins[&basin_up_key.unwrap()];
                let basin_left = basins[&basin_left_key.unwrap()];
                if basin_up != basin_left {
                    basin_sizes[basin_up] += basin_sizes[basin_left];
                    basin_sizes[basin_left] = 0;
                    *basins.get_mut(&basin_left_key.unwrap()).unwrap() = basin_up;
                }
            }

            if basin_up_exists {
                let basin = basins[&basin_up_key.unwrap()];
                basins.insert(*basin_key, basin);
                basin_sizes[basin] += 1;
            } else if basin_left_exists {
                let basin = basins[&basin_left_key.unwrap()];
                basins.insert(*basin_key, basin);
                basin_sizes[basin] += 1;
            } else {
                basins.insert(*basin_key, basin_count);
                basin_sizes.push(1);
                basin_count += 1;
            }
        }
    }

    basin_sizes.sort();
    let result: i64 = basin_sizes.iter().rev().take(3).copied().reduce(|a, b| a * b).unwrap();

    // println!("basin count: {}", basin_count);
    // println!("basin sizes: {:?}", &basin_sizes);
    // for (row_index, row) in map.iter().enumerate() {
    //     for (column_index, _) in row.iter().copied().enumerate() {
    //         match basins.get_mut(&(row_index, column_index)) {
    //             Some(basin) => print!("{}", basin),
    //             None => print!("."),
    //         };
    //     }
    //     println!();
    // }

    return result;
}

fn calculate_risk_level(input: &str) -> i64 {
    let map = parse_input(input);
    let mut risk_level = 0;
    let map_width = map[0].len();
    for (row_index, row) in map.iter().enumerate() {
        for (column_index, height) in row.iter().copied().enumerate() {
            if height < 9 && is_lowest(&map, map_width, row_index, column_index) {
                risk_level += height as i64 + 1;
            }
        }
    }

    return risk_level;
}

fn is_lowest(map: &Vec<Vec<u8>>, map_width: usize, row_index: usize, column_index: usize) -> bool {
    let height = map[row_index][column_index];
    let mut is_lowest = true;

    if row_index > 0 {
        let height_up = map[row_index - 1][column_index];
        is_lowest &= height < height_up;
    }
    if is_lowest && column_index > 0 {
        let height_left = map[row_index][column_index - 1];
        is_lowest &= height < height_left;
    }
    if is_lowest && row_index < map.len() - 1 {
        let height_down = map[row_index + 1][column_index];
        is_lowest &= height < height_down;
    }
    if is_lowest && column_index < map_width - 1 {
        let height_right = map[row_index][column_index + 1];
        is_lowest &= height < height_right;
    }

    return is_lowest;
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    return input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|char| char - '0' as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "2199943210
3987894921
9856789892
8767896789
9899965678";
    }

    #[test]
    fn example_part1_correct_result() {
        let result = calculate_risk_level(get_example_input());

        assert_eq!(15, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(607, result);
    }

    #[test]
    fn example_part2_correct_result() {
        let result = calculate_largest_basins(get_example_input());

        assert_eq!(1134, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(900864, result);
    }
}
