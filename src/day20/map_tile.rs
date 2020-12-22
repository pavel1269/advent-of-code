#[path = "map_tile.determine_map_change.tests.rs"]
#[cfg(test)]
mod tests;
use super::directions::*;

#[derive(Debug)]
pub struct MapTile {
    pub id: i64,
    pub map: String,
    pub edges: [(String, String); 4],
}

impl MapTile {
    pub fn edge(&self, direction: Directions) -> &String {
        // println!("Asking for '{:?}', all edges: {:?}", direction, &self.edges);
        let direction = direction.index();
        let edges = &self.edges[direction];
        return &edges.0;
    }

    pub fn set_map_way(
        &mut self,
        edge_index: usize,
        matches_mirrored: bool,
        search_direction: Directions,
    ) {
        let (rotate_times, mirror_x, mirror_y) =
            Self::determine_map_change(edge_index, matches_mirrored, search_direction);
        // println!(
        //     "[{}] Set rotated to {}, mirroring x: {} y: {} ({}, {}, {:?})",
        //     self.id,
        //     rotate_times,
        //     mirror_x,
        //     mirror_y,
        //     edge_index,
        //     matches_mirrored,
        //     search_direction,
        // );
        let new_map = self.rotated_map(rotate_times, mirror_x, mirror_y);
        *self = Self::from(self.id, new_map.as_str());
    }

    fn determine_map_change(
        edge_index: usize,
        matches_mirrored: bool,
        search_direction: Directions,
    ) -> (usize, bool, bool) {
        let rotate_times = (Directions::count() + search_direction.match_offset() - edge_index)
            % Directions::count();
        let mirror_x = !matches_mirrored
            && (search_direction == Directions::Up || search_direction == Directions::Down);
        let mirror_y = !matches_mirrored
            && (search_direction == Directions::Left || search_direction == Directions::Right);

        return (rotate_times, mirror_x, mirror_y);
    }

    fn rotated_map(&self, rotate_times: usize, mut mirror_x: bool, mirror_y: bool) -> String {
        let result = if rotate_times == 0 {
            self.map.clone()
        } else if rotate_times == 2 {
            self.map
                .lines()
                .rev()
                .collect::<Vec<&str>>()
                .iter()
                .map(|line| line.chars().rev().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            let map_rows = if rotate_times == 1 {
                mirror_x = !mirror_x;
                self.map
                    .lines()
                    .map(|line| line.chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>()
            } else {
                self.map
                    .lines()
                    .map(|line| line.chars().rev().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>()
            };

            let len = map_rows[0].len();
            let mut new_rows: Vec<Vec<char>> = Vec::with_capacity(len);

            for _ in 0..len {
                new_rows.push(Vec::with_capacity(len));
            }
            for index_x in 0..len {
                for index_y in 0..len {
                    new_rows[index_y].push(map_rows[index_x][index_y]);
                }
            }

            new_rows
                .iter()
                .map(|line| line.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        };

        let result = if mirror_y {
            result.lines().rev().collect::<Vec<&str>>().join("\n")
        } else if mirror_x {
            result
                .lines()
                .map(|line| line.chars().rev().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            result
        };

        return result;
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
        }
    }
}
