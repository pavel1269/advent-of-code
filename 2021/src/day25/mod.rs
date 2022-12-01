use std::convert::TryInto;


pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = stabilize_crabs(input);
    return result;
}

fn stabilize_crabs(input: &str) -> i64 {
    let mut map = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let height = map.len();
    let width = map[0].len();
    let mut iterations = 0;
    let mut changes = 1;
    while changes > 0 {
        changes = 0;
        iterations += 1;

        map = map.iter().map(|line| {
            let mut line_new = line.clone();
            for index in 0..width {
                if line[index] != '>' {
                    continue;
                }
                
                let index_right = if index + 1 == width {
                    0
                } else {
                    index + 1
                };

                let right = line.get(index_right).unwrap();
                if *right == '.' {
                    *line_new.get_mut(index_right).unwrap() = '>';
                    *line_new.get_mut(index).unwrap() = '.';
                    changes += 1;
                }
            }

            line_new
        }).collect::<Vec<_>>();

        let mut map_new = map.clone();
        for index_column in 0..width {
            for index_row in 0..height {
                if map[index_row][index_column] != 'v' {
                    continue;
                }

                let index_down = if index_row + 1 == height {
                    0
                } else {
                    index_row + 1
                };

                let down = map.get(index_down).unwrap().get(index_column).unwrap();
                if *down == '.' {
                    *map_new.get_mut(index_down).unwrap().get_mut(index_column).unwrap() = 'v';
                    *map_new.get_mut(index_row).unwrap().get_mut(index_column).unwrap() = '.';
                    changes += 1;
                }
            }
        }
        map = map_new;

        // println!("map [{}]:", iterations);
        // for line in map.iter().map(|line| line.into_iter().collect::<String>()) {
        //     println!("{}", line);
        // }
        // println!();
    }
    
    return iterations;
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"
    }

    #[test]
    fn example_stabilize_test() {
        let result = stabilize_crabs(get_example_input());
        assert_eq!(58, result);
    }

    #[test]
    fn part1_test() {
        let result = get_solution_part1();
        assert_eq!(601, result);
    }
}
