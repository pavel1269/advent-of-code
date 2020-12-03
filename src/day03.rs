
pub fn get_solution_part1() -> i32 {
    let input = get_challenge_input();
    let map = parse_input(input);
    let result = count_hit_trees(map, 3, 1);

    return result;
}

fn count_hit_trees(map: Vec<Vec<char>>, move_right: usize, move_down: usize) -> i32 {
    let mut hit_trees = 0;

    let mut position_x = 0;
    let mut position_y = 0;
    let map_height = map.len();
    let map_width = map[0].len();

    while position_y < map_height {
        let map_field = map[position_y][position_x];

        if map_field == '#' {
            hit_trees += 1;
        }

        position_x = (position_x + move_right) % map_width;
        position_y += move_down;
    }

    return hit_trees;
}

fn parse_input(input: Vec<&str>) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect()).collect()
}

fn get_challenge_input() -> Vec<&'static str> {
    include_str!("./inputs/day03.txt").lines().collect()
}

#[allow(dead_code)]
fn get_example_input() -> Vec<&'static str> {
    "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#".lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = get_example_input();
        let map = parse_input(input);
        let result = count_hit_trees(map, 3, 1);

        assert_eq!(7, result);
    }

    #[test]
    fn day01_part1() {
        let expected_result = 274;
        let result = get_solution_part1();

        assert_eq!(expected_result, result);
    }
}