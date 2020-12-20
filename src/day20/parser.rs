use super::directions::*;

#[derive(Debug)]
pub struct MapTile {
    id: i64,
    map: String,
    edges: [(String, String); 4],
    connections: [Option<i64>; 4],
}

impl MapTile {
    pub fn from(id: i64, map: &str) -> Self {
        let mut edges = [
            (String::new(), String::new()),
            (String::new(), String::new()),
            (String::new(), String::new()),
            (String::new(), String::new()),
        ];

        let map = map.trim();
        let mut map_lines = map.lines();
        let first_line = map_lines.next().unwrap();
        let len: usize = first_line.len();
        debug_assert!(len > 0);
        edges[Directions::Up.index()].0 = String::from(first_line);
        edges[Directions::Up.index()].1 = first_line.chars().rev().collect();

        let last_line = map_lines.last().unwrap();
        debug_assert_eq!(len, last_line.len());
        edges[Directions::Down.index()].0 = String::from(last_line);
        edges[Directions::Down.index()].1 = last_line.chars().rev().collect();

        for line in map.lines() {
            let mut line_iter = line.chars().into_iter();

            edges[Directions::Left.index()]
                .0
                .push(line_iter.next().unwrap());
            edges[Directions::Right.index()]
                .0
                .push(line_iter.last().unwrap());
        }

        debug_assert_eq!(len, edges[Directions::Left.index()].0.len());
        edges[Directions::Left.index()].1 =
            edges[Directions::Left.index()].0.chars().rev().collect();
        edges[Directions::Right.index()].1 =
            edges[Directions::Right.index()].0.chars().rev().collect();

        Self {
            id: id,
            map: String::from(map),
            edges: edges,
            connections: [None, None, None, None],
        }
    }
}

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
