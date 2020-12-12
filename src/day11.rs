
pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let map = simulate_till_still(input);
    let result = count_occupied_seats(&map);

    return result;
}

#[derive(Clone)]
#[derive(PartialEq)]
enum Layout {
    Floor,
    Free,
    Occupied,
}

fn count_occupied_seats(map: &Vec<Vec<Layout>>) -> i64 {
    let mut occupied: i64 = 0;
    for occupants in map.iter().map(|row| row.iter().filter(|space| **space == Layout::Occupied).count()) {
        occupied += occupants as i64;
    }

    return occupied;
}

fn simulate_till_still(input: &str) -> Vec<Vec<Layout>> {
    let mut map_now = parse_input(input);
    let height = map_now.len();
    let width = map_now[0].len();
    loop {
        let mut map_next = map_now.clone();
        let mut any_change = false;
        for row in 0..height {
            for column in 0..width {
                let occupied = get_occupied_around(&map_now, row as i32, column as i32, height, width);
                if map_now[row][column] == Layout::Occupied {
                    if occupied >= 4 {
                        map_next[row][column] = Layout::Free;
                        any_change = true;
                    }
                } else if map_now[row][column] == Layout::Free {
                    if occupied == 0 {
                        map_next[row][column] = Layout::Occupied;
                        any_change = true;
                    }
                }
            }
        }

        // print_map(&map_next);

        if !any_change {
            return map_now;
        }

        map_now = map_next;
    }
}

fn get_occupied_around(map: &Vec<Vec<Layout>>, row: i32, column: i32, height: usize, width: usize) -> u8 {
    let mut occupied = 0;
    for row_offset in -1..2 {
        for column_offset in -1..2 {
            if row_offset == 0 && column_offset == 0 {
                continue;
            }
            // println!("testing [{}][{}]", column_offset, row_offset);

            let row = row + row_offset;
            let column = column + column_offset;

            if row < 0 || column < 0 || row >= height as i32 || column >= width as i32 {
                // println!("[{}][{}] out of map", column, row);
                continue;
            }

            if map[row as usize][column as usize] == Layout::Occupied {
                // println!("[{}][{}] = occupied", column, row);
                occupied += 1;
            } else {
                // println!("[{}][{}] = free", column, row);
            }
        }
    }

    return occupied;
}

fn parse_input(input: &str) -> Vec<Vec<Layout>> {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .to_vec()
                .iter()
                .map(|char| match *char as char {
                    '.' => Layout::Floor,
                    'L' => Layout::Free,
                    '#' => Layout::Occupied,
                    _ => panic!("Unexpected map token"),
                })
                .collect::<Vec<Layout>>()
        })
        .collect::<Vec<Vec<Layout>>>()
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<Layout>>) {
    println!("Map:");
    for row in map {
        println!("{}", row.iter().map(|char| match char {
            Layout::Floor => ".",
            Layout::Free => "L",
            Layout::Occupied => "#",
        }).collect::<Vec<&str>>().join(""));
    }
    println!("");
}

fn get_challenge_input() -> &'static str {
    include_str!("./inputs/day11.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
        }
        
    fn get_example2_input() -> &'static str {
".L#
#.L
#L#"
    }

    #[test]
    fn example2_get_occupied_around() {
        let input = get_example2_input();
        let map = parse_input(input);

        print_map(&map);

        let result = (
            (
                get_occupied_around(&map, 0, 0, 3, 3),
                get_occupied_around(&map, 0, 1, 3, 3),
                get_occupied_around(&map, 0, 2, 3, 3),
            ),
            (
                get_occupied_around(&map, 1, 0, 3, 3),
                get_occupied_around(&map, 1, 1, 3, 3),
                get_occupied_around(&map, 1, 2, 3, 3),
            ),
            (
                get_occupied_around(&map, 2, 0, 3, 3),
                get_occupied_around(&map, 2, 1, 3, 3),
                get_occupied_around(&map, 2, 2, 3, 3),
            ),
        );

        assert_eq!((
            (1, 2, 0),
            (1, 4, 2),
            (1, 3, 0),
        ), result);
    }

    #[test]
    fn example_part1_result() {
        let input = get_example_input();
        let map = simulate_till_still(input);
        let result = count_occupied_seats(&map);

        assert_eq!(37, result);
    }

    #[test]
    fn input_part1_result() {
        let result = get_part1_result();

        assert_eq!(2247, result);
    }
}
