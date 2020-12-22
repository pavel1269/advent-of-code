mod directions;
mod map_connector;
mod parser;
mod map_tile;
use map_connector::*;
use parser::*;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let corners_sum = multiply_map_corner_ids(input);

    return corners_sum;
}

fn multiply_map_corner_ids(input: &str) -> i64 {
    let mut map = parse_input(input);
    let result_map = connect_map(&mut map);

    let map_dimension = result_map.len();
    let mut corners_sum: i64 = 1;
    corners_sum *= map[result_map[0][0]].id;
    corners_sum *= map[result_map[0][map_dimension - 1]].id;
    corners_sum *= map[result_map[map_dimension - 1][0]].id;
    corners_sum *= map[result_map[map_dimension - 1][map_dimension - 1]].id;

    return corners_sum;
}

fn get_challenge_input() -> &'static str {
    include_str!("input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn get_example_input() -> &'static str {
        include_str!("example.txt")
    }
    
    #[test]
    fn example_part1_result() {
        let input = get_example_input();
        let result = multiply_map_corner_ids(input);

        assert_eq!(20899048083289, result);
    }

    #[test]
    fn input_part1_result() {
        let input = get_challenge_input();
        let result = multiply_map_corner_ids(input);

        assert_eq!(32287787075651, result);
    }
}
