
pub fn get_solution_part1() -> i64 {
    let input = get_challenge_input();
    let map = parse_input(input);
    let result = count_hit_trees(&map, 3, 1);

    return result as i64;
}

pub fn get_solution_part2() -> i64 {
    let input = get_challenge_input();
    let map = parse_input(input);
    let moves = get_part2_moves();
    let result = count_hit_trees_multiple(&map, moves);

    return result;
}

fn get_part2_moves() -> Vec<(usize, usize)> {
    vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ]
}

fn count_hit_trees_multiple(map: &Vec<Vec<char>>, moves: Vec<(usize, usize)>) -> i64 {
    let mut result_total: i64 = 1;
    for move_def in moves.iter() {
        let result = count_hit_trees(&map, move_def.0, move_def.1);
        result_total *= result as i64;
    };

    return result_total;
}

fn count_hit_trees(map: &Vec<Vec<char>>, move_right: usize, move_down: usize) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

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
    
    #[test]
    fn example_part1_correct_result() {
        let input = get_example_input();
        let map = parse_input(input);
        let result = count_hit_trees(&map, 3, 1);

        assert_eq!(7, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let expected_result = 274;
        let result = get_solution_part1();

        assert_eq!(expected_result, result);
    }

    #[test]
    fn example_part2_correct_result() {
        let input = get_example_input();
        let map = parse_input(input);
        let moves = get_part2_moves();
        let result = count_hit_trees_multiple(&map, moves);
        
        assert_eq!(336, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let expected_result = 6050183040;
        let result = get_solution_part2();

        assert_eq!(expected_result, result);
    }
}
