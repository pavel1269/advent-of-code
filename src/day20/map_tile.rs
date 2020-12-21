use super::directions::*;

#[derive(Debug)]
pub struct MapTile {
    pub id: i64,
    map: String,
    pub edges: [(String, String); 4],
    // Clockwise roration
    pub rotated_times: usize,
    pub mirrored: bool,
}

impl MapTile {
    pub fn edge(&self, direction: Directions, mirrored: bool) -> &String {
        println!("Asking for '{:?}' and mirrored: {}", direction, mirrored);
        let direction = (direction.index() + self.rotated_times) % Directions::count();
        let edges = &self.edges[direction];
        if mirrored ^ self.mirrored {
            println!("Returning index '{}' and mirrored: true", direction);
            return &edges.1;
        } else {
            println!("Returning index '{}' and mirrored: false", direction);
            return &edges.0;
        }
    }

    pub fn set_match_way(&mut self, other_mirrored: bool, other_rotated_times: usize, edge_index: usize, matches_mirrored: bool, search_direction: Directions) {
        // a = other_mirrored
        // b = matches_mirrored
        // f = new mirrored
        // a b f
        // 0 0 0
        // 0 1 1
        // 1 0 1
        // 1 1 0
        self.mirrored = other_mirrored ^ matches_mirrored;
        self.rotated_times = (other_rotated_times + edge_index + search_direction.match_offset()) % Directions::count();
        println!("[{}] Set rotated to {} ({} + {} + {}), mirrored {}", self.id, self.rotated_times, other_rotated_times, edge_index, search_direction.match_offset(), self.mirrored);
    }

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
        edges[Directions::Down.index()].1 = String::from(last_line);
        edges[Directions::Down.index()].0 = last_line.chars().rev().collect();

        for line in map.lines() {
            let mut line_iter = line.chars().into_iter();

            edges[Directions::Left.index()]
                .1
                .push(line_iter.next().unwrap());
            edges[Directions::Right.index()]
                .0
                .push(line_iter.last().unwrap());
        }

        edges[Directions::Left.index()].0 =
            edges[Directions::Left.index()].1.chars().rev().collect();
        edges[Directions::Right.index()].1 =
            edges[Directions::Right.index()].0.chars().rev().collect();
        debug_assert_eq!(len, edges[Directions::Left.index()].0.len());
        debug_assert_eq!(len, edges[Directions::Left.index()].1.len());
        debug_assert_eq!(len, edges[Directions::Right.index()].0.len());
        debug_assert_eq!(len, edges[Directions::Right.index()].1.len());

        Self {
            id: id,
            map: String::from(map),
            edges: edges,
            rotated_times: 0,
            mirrored: false,
        }
    }
}
