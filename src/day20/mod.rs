mod directions;
mod map_connector;
mod map_creator;
mod map_tile;
mod monster_finder;
mod parser;
use map_connector::*;
use map_creator::*;
use monster_finder::*;
use parser::*;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let corners_sum = multiply_map_corner_ids(input);

    return corners_sum;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let roughness = calculate_roughness(input);

    return roughness;
}

fn calculate_roughness(input: &str) -> i64 {
    use map_tile::*;
    use directions::*;

    let image = create_image(input);

    // Rotate so monsters are not vertically
    let mut tile = MapTile::from(0, image.join("\n").as_str());
    tile.set_map_way(0, false, Directions::Left);
    let image = tile.map.lines().map(|line| String::from(line)).collect::<Vec<String>>();

    println!("{}", &image.join("\n"));

    let monster_patterns = get_monster_patterns();
    let matches = count_monsters(&image, &monster_patterns);
    let matches_count = matches.iter().sum::<i64>();
    let monster_size = monster_patterns[0].get_size() as i64;
    let monsters_size = matches_count * monster_size;
    let roughness_total = image.join("").chars().filter(|char| *char == '#').count() as i64;
    let roughness = roughness_total - monsters_size;
    println!("Matches: {}, monster size: {}, roughness total: {}, roughness: {}", matches_count, monster_size, roughness_total, roughness);

    return roughness;
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

    #[test]
    fn example_part2_result() {
        let input = get_example_input();
        let result = calculate_roughness(input);

        assert_eq!(273, result);
    }

    #[test]
    fn input_part2_result() {
        let input = get_challenge_input();
        let result = calculate_roughness(input);

        assert_eq!(1939, result);
    }
}
