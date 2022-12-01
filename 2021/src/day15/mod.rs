use std::collections::{hash_map::Entry, HashMap};

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = cheapest_path(input, 1);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = cheapest_path(input, 5);
    return result;
}

fn cheapest_path(input: &str, enlarge_times: usize) -> i64 {
    let (mut width, mut height, mut map) = parse_input(input);
    if enlarge_times > 1 {
        let (new_width, new_height, new_map) = enlarge_map(map, width, height, enlarge_times);
        width = new_width;
        height = new_height;
        map = new_map;
    }

    let width = width;
    let height = height;
    let map = map;

    let width_bound = width - 1;
    let height_bound = height - 1;
    const STEP_COST: i64 = 1;
    let mut path = vec![((0, 0), 0, (width_bound + height_bound) as i64 * STEP_COST); 1];
    let mut visited = HashMap::new();

    while let Some(((x, y), cost, distance)) = path.pop() {
        match visited.entry((x, y)) {
            Entry::Vacant(entry) => {
                entry.insert(cost);
            }
            Entry::Occupied(mut visited_cost) => {
                if *visited_cost.get() <= cost {
                    continue;
                } else {
                    *visited_cost.get_mut() = cost;
                }
            }
        };

        if x == width - 1 && y == height - 1 {
            return cost;
        }
        if x + 1 < width {
            path.push(((x + 1, y), cost + map[y][x + 1], distance - STEP_COST));
        }
        if y + 1 < height {
            path.push(((x, y + 1), cost + map[y + 1][x], distance - STEP_COST));
        }
        if x > 0 {
            path.push(((x - 1, y), cost + map[y][x - 1], distance + STEP_COST));
        }
        if y > 0 {
            path.push(((x, y - 1), cost + map[y - 1][x], distance + STEP_COST));
        }

        path.sort_unstable_by_key(|(_, cost, distance)| *cost + *distance);
        path.reverse();
    }

    panic!();
}

fn enlarge_map(map: Vec<Vec<i64>>, width: usize, height: usize, times: usize) -> (usize, usize, Vec<Vec<i64>>) {
    let mut map_new = vec![Vec::with_capacity(width * times); height * times];
    // times -> offsets
    // 1 -> 0
    // 2 -> 0,1,2
    // 3 -> 0,1,2,3,4
    // 4 -> 0,1,2,3,4,5,6
    for offset in 0..(times - 1) * 2 + 1 {
        let map_new_suqare = map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cost| {
                        let mut cost = *cost + offset as i64;
                        while cost > 9 {
                            cost -= 9;
                        }
                        return cost;
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // times: 5, offset:
        // 0 -> 0
        // 1 -> 0,1
        // 2 -> 0,1,2
        // 3 -> 0,1,2,3
        // 4 -> 0,1,2,3,4
        // 5 -> 1,2,3,4
        // 6 -> 2,3,4
        // 7 -> 3,4
        // 8 -> 4
        // max(0, offset - times + 1) .. min(offset + 1, times)
        let start = std::cmp::max(0 as i32, offset as i32 - times as i32 + 1) as usize;
        let end = std::cmp::min(offset + 1, times);
        for box_offset in start..end {
            for (row_index, row) in map_new_suqare.iter().enumerate() {
                //map_new[box_offset * height + row_index].extend_from_slice(row);
                map_new[box_offset * height + row_index].extend(row);
            }
        }
    }

    return (width * times, height * times, map_new);
}

fn parse_input(input: &str) -> (usize, usize, Vec<Vec<i64>>) {
    let mut width = None;
    let mut map = Vec::new();
    for line in input.lines() {
        let row = line
            .as_bytes()
            .into_iter()
            .map(|c| (c - '0' as u8) as i64)
            .collect::<Vec<_>>();
        width = Some(row.len());
        map.push(row);
    }
    let width = width.unwrap();
    let height = map.len();

    return (width, height, map);
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_1_input() -> &'static str {
        return "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    }

    fn get_example_2_input() -> &'static str {
        return "19999
19111
11191";
    }

    #[test]
    fn example1_part1_correct_result() {
        let result = cheapest_path(get_example_1_input(), 1);

        assert_eq!(40, result);
    }

    #[test]
    fn example2_part1_correct_result() {
        let result = cheapest_path(get_example_2_input(), 1);

        assert_eq!(8, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(462, result);
    }

    // #[test]
    // fn example1_enlarge_map() {
    //     let (width, height, map) = parse_input(get_example_1_input());
    //     let (width, height, map) = enlarge_map(map, width, height, 5);

    //     assert_eq!(50, width);
    //     assert_eq!(50, height);
    //     assert_eq!(50, map.len());

    //     for row in map.iter() {
    //         for char in row.iter() {
    //             print!("{}", char);
    //         }
    //         println!();
    //     }

    //     assert!(false);
    // }

    #[test]
    fn example1_part2_correct_result() {
        let result = cheapest_path(get_example_1_input(), 5);

        assert_eq!(315, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(2846, result);
    }
}
