use super::map_tile::*;

pub fn parse_input(input: &str) -> Vec<MapTile> {
    let mut parts: Vec<MapTile> = Vec::new();
    for part in input.split("Tile") {
        if part.trim().len() == 0 {
            continue;
        }

        let result = part.splitn(2, ":").collect::<Vec<&str>>();
        let id = result[0].trim().parse::<i64>().unwrap();
        let tile = MapTile::from(id, result[1]);
        parts.push(tile);
    }
    return parts;
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::super::tests::get_example_input;
    use super::*;

    #[test]
    fn example_parse_input_count_matches() {
        let input = get_example_input();
        let input = parse_input(input);

        assert_eq!(9, input.len());
    }

    #[test]
    fn input_parse_input_count_matches() {
        let input = get_challenge_input();
        let input = parse_input(input);

        assert_eq!(144, input.len());
    }
}
