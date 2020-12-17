use super::coordinates::*;

#[derive(Clone)]
pub struct Space {
    dimensions: Vec<bool>,
    dimension_count: usize,
    coordinates: Coordinates,
}

impl Space {
    pub fn simlate_cycle(&mut self) {
        let mut expand = vec![(false, false); self.dimension_count];
        let mut dimensions_new = Self::create_dimensions_coordinates(&self.coordinates);

        for index in 0..self.coordinates.max_index() {
            let location = Coordinates::from_index(index, &self.coordinates);
            let around = self.count_active_surrounding(&location);
            let value = self.dimensions[index];

            // println!("index: {}, loc: {:?}, around: {}", index, location, around);
            let mut changed = false;
            if value {
                if around >= 2 && around <= 3 {
                    // println!("index: {}, keep true", index);
                    dimensions_new[index] = true;
                    changed = true;
                }
            } else {
                if around == 3 {
                    // println!("index: {}, set true", index);
                    dimensions_new[index] = true;
                    changed = true;
                }
            }

            if changed {
                for coord_index in 0..self.dimension_count {
                    let dim_index = *location.index(coord_index);
                    if dim_index == 0 {
                        expand[coord_index].0 = true;
                    }
                    if dim_index >= *self.coordinates.index(coord_index) - 1 {
                        expand[coord_index].1 = true;
                    }
                }
            }
        }

        self.dimensions = dimensions_new;
        self.expand_if_needed(&expand);
    }

    fn expand_if_needed(&mut self, expand: &Vec<(bool, bool)>) {
        print!("{:?}", expand);
        let mut new_coordinates = self.coordinates.clone();
        for (dimension, expand) in expand.iter().enumerate() {
            if expand.0 {
                *new_coordinates.index_mut(dimension) += 1;
            }
            if expand.1 {
                *new_coordinates.index_mut(dimension) += 1;
            }
        }

        let mut new_dimensions = Space::create_dimensions_coordinates(&new_coordinates);
        for index in 0..self.dimensions.len() {
            let mut coords = Coordinates::from_index(index, &self.coordinates);

            for (dimension, expand) in expand.iter().enumerate() {
                if expand.0 {
                    *coords.index_mut(dimension) += 1;
                }
            }

            let new_index = coords.to_index(&new_coordinates);
            new_dimensions[new_index] = self.dimensions[index];
        }

        self.dimensions = new_dimensions;
        self.coordinates = new_coordinates;
    }

    fn count_active_surrounding(&self, location: &Coordinates) -> usize {
        let combinations = usize::pow(3, self.dimension_count as u32);
        let mut surrounding: Vec<i32> = vec![-1; self.dimension_count];
        surrounding[0] -= 1;
        let mut count: usize = 0;
        for _ in 0..combinations {
            surrounding[0] += 1;
            for index in 0..self.dimension_count {
                if surrounding[index] > 1 {
                    surrounding[index + 1] += 1;
                    surrounding[index] = -1;
                }
            }

            let location_to_check = location.sub(&surrounding, &self.coordinates);
            match location_to_check {
                SubResult::Overflow => {
                    // println!("{:?} - {:?} = out of map", location, surrounding);
                }
                SubResult::Result(location_to_check) => {
                    if &location_to_check == location {
                        continue;
                    }

                    // println!("{:?} - {:?} = {:?}", location, surrounding, location_to_check);
                    if self.at(&location_to_check) {
                        count += 1;
                    }
                }
            }
        }

        return count;
    }

    fn at(&self, coordinates: &Coordinates) -> bool {
        let index = coordinates.to_index(&self.coordinates);
        let result = self.dimensions[index];
        return result;
    }

    pub fn count_actives(&self) -> u32 {
        let mut sum: u32 = 0;
        for field in self.dimensions.iter() {
            if *field {
                sum += 1;
            }
        }
        return sum;
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{:?}", self.coordinates);

        let mut line = String::new();
        let mut abstract_indexes = vec![0; self.dimension_count - 2];
        for (index, item) in self.dimensions.iter().enumerate() {
            if index > 0 && index % *self.coordinates.index(1) == 0 {
                println!("{}: {}", index, line);
            }
            if index % *self.coordinates.index(0) == 0 {
                line.clear();
            }
            if *item {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("_: {}", line);
    }

    pub fn from_input(input: &str, dimensions: usize) -> Space {
        let mut input_lines = input.lines();
        let max_x = input_lines.next().unwrap().len() + 2;
        let max_y = input_lines.count() + 3;
        let mut space = Space::create_from_2d(max_x, max_y, dimensions);

        let mut offset = 0;
        let mut multiplier = *space.coordinates.index(0);
        for dim_index in 1..dimensions - 1 {
            multiplier *= space.coordinates.index(dim_index);
            offset += multiplier;
        }

        let multiplier = *space.coordinates.index(1);
        for (index_y, line) in input.lines().enumerate() {
            if line.len() + 2 != max_x {
                panic!(format!(
                    "Unexpected input dimension, found {}, expected: {}",
                    line.len(),
                    max_x
                ));
            }

            for (index_x, char) in line.chars().enumerate() {
                match char {
                    '.' => {}
                    '#' => {
                        let index = index_x + 1 + (index_y + 1) * multiplier + offset;
                        space.dimensions[index] = true;
                    }
                    _ => panic!("Unexpected input char"),
                }
            }
        }

        return space;
    }

    fn create_dimensions_coordinates(coordinates: &Coordinates) -> Vec<bool> {
        let max_index = coordinates.max_index();
        let dimensions_vec = vec![false; max_index];
        return dimensions_vec;
    }

    fn create_from_2d(x: usize, y: usize, dimension_count: usize) -> Self {
        let mut dimensions = vec![x, y];
        while dimensions.len() < dimension_count {
            dimensions.push(3);
        }
        let max_index = Coordinates::from(&dimensions).max_index();
        let dimensions_vec = vec![false; max_index];
        Self {
            dimensions: dimensions_vec,
            dimension_count: dimension_count,
            coordinates: Coordinates::from(&dimensions),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        ".#.
..#
###"
    }

    #[test]
    fn direct_example_parse_dimensions() {
        let space = Space::from_input("#", 3);

        let expected_result = vec![
            false, false, false,
            false, false, false,
            false, false, false,
            
            false, false, false,
            false, true, false,
            false, false, false,
            
            false, false, false,
            false, false, false,
            false, false, false,
        ];
        assert_eq!(&expected_result, &space.dimensions);
    }

    #[test]
    fn example_parse_dimensions() {
        let input = get_example_input();
        let space = Space::from_input(input, 3);

        let expected_result = vec![
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            
            false, false, false, false, false,
            false, false,  true, false, false,
            false, false, false,  true, false,
            false,  true,  true,  true, false,
            false, false, false, false, false,
            
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
        ];
        assert_eq!(&expected_result, &space.dimensions);
    }

    #[test]
    fn example_parse_coordinates() {
        let input = get_example_input();
        let space = Space::from_input(input, 3);

        assert_eq!(Coordinates::from(&vec![5, 5, 3]), space.coordinates);
    }

    #[test]
    fn direct_example_parse_coordinates() {
        let space = Space::from_input("..#\n##.", 3);

        assert_eq!(Coordinates::from(&vec![5, 4, 3]), space.coordinates);
    }

    #[test]
    fn example_parsed_active_count() {
        let input = get_example_input();
        let space = Space::from_input(input, 3);

        let result = space.count_actives();

        assert_eq!(5, result);
    }

    #[test]
    fn example_parsed_count_active_surrounding() {
        let input = get_example_input();
        let space = Space::from_input(input, 3);

        let result = space.count_active_surrounding(&Coordinates::from(&vec![3, 1, 1]));

        assert_eq!(2, result);
    }

    #[test]
    fn example_parsed_count_active_surrounding_upper_edge() {
        let input = get_example_input();
        let space = Space::from_input(input, 3);

        let result = space.count_active_surrounding(&Coordinates::from(&vec![2, 0, 1]));

        assert_eq!(1, result);
    }

    #[test]
    fn example_parsed_count_active_surrounding_bottom_right_corner() {
        let input = get_example_input();
        let space = Space::from_input(input, 3);

        let result = space.count_active_surrounding(&Coordinates::from(&vec![0, 4, 0]));

        assert_eq!(1, result);
    }

    #[test]
    fn example_parsed_count_active_surrounding_active_place() {
        let input = get_example_input();
        let space = Space::from_input(input, 3);

        let result = space.count_active_surrounding(&Coordinates::from(&vec![2, 1, 1]));

        assert_eq!(1, result);
    }

    #[test]
    fn example_parsed_simlate_cycle_count_actives() {
        let input = get_example_input();
        let mut space = Space::from_input(input, 3);

        space.simlate_cycle();
        let result = space.count_actives();

        assert_eq!(11, result);
    }

    #[test]
    fn example_parsed_simlate_cycle_grew() {
        let input = get_example_input();
        let mut space = Space::from_input(input, 3);

        space.simlate_cycle();

        assert_eq!(Coordinates::from(&vec![5, 6, 5]), space.coordinates);
    }

    #[test]
    fn example_parsed_simlate_cycle_slive_matches() {
        let input = get_example_input();
        let mut space = Space::from_input(input, 3);

        space.simlate_cycle();

        assert_eq!(vec![
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            
            false, false, false, false, false,
            false, false, false, false, false,
            false,  true, false, false, false,
            false, false, false,  true, false,
            false, false,  true, false, false,
            false, false, false, false, false,
            
            false, false, false, false, false,
            false, false, false, false, false,
            false,  true, false,  true, false,
            false, false,  true,  true, false,
            false, false,  true, false, false,
            false, false, false, false, false,
            
            false, false, false, false, false,
            false, false, false, false, false,
            false,  true, false, false, false,
            false, false, false,  true, false,
            false, false,  true, false, false,
            false, false, false, false, false,
            
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
            false, false, false, false, false,
        ], space.dimensions);
    }

    #[test]
    fn example_parsed_simlate_cycle2_count_actives() {
        let input = get_example_input();
        let mut space = Space::from_input(input, 3);

        space.simlate_cycle();
        space.simlate_cycle();
        let result = space.count_actives();

        assert_eq!(21, result);
    }

    #[test]
    fn example_parsed_simlate_cycle2_grew() {
        let input = get_example_input();
        let mut space = Space::from_input(input, 3);

        space.simlate_cycle();
        space.simlate_cycle();

        assert_eq!(Coordinates::from(&vec![7, 7, 7]), space.coordinates);
    }
}
