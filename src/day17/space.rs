use super::coordinates::*;

#[derive(Clone)]
pub struct Space {
    dimensions: Vec<Vec<Vec<bool>>>,
    coordinates: Coordinates,
}

impl Space {
    pub fn simlate_cycle(&mut self) {
        let mut expand_x = (false, false);
        let mut expand_y = (false, false);
        let mut expand_z = (false, false);

        let mut dimensions_new = Self::create_dimensions_coordinates(&self.coordinates);
        for index_z in 0..self.coordinates.z {
            let dim_xy = &self.dimensions[index_z];
            let dim_xy_new = &mut dimensions_new[index_z];
            for index_y in 0..self.coordinates.y {
                let dim_x = &dim_xy[index_y];
                let dim_x_new = &mut dim_xy_new[index_y];
                for (index_x, value_new) in dim_x_new.iter_mut().enumerate() {
                    let around = self.count_active_surrounding(index_x, index_y, index_z);
                    // println!("{} {} {} = {}", index_x, index_y, index_z, around);
                    let mut changed = false;
                    let value = dim_x[index_x];
                    if value {
                        if around >= 2 && around <= 3 {
                            *value_new = true;
                            changed = true;
                        }
                    } else {
                        if around == 3 {
                            *value_new = true;
                            changed = true;
                        }
                    }

                    if changed {
                        if index_x == 0 {
                            expand_x.0 = true;
                        }
                        if index_x >= self.coordinates.x - 1 {
                            expand_x.1 = true;
                        }
                        if index_y == 0 {
                            expand_y.0 = true;
                        }
                        if index_y >= self.coordinates.y - 1 {
                            expand_y.1 = true;
                        }
                        if index_z == 0 {
                            expand_z.0 = true;
                        }
                        if index_z >= self.coordinates.z - 1 {
                            expand_z.1 = true;
                        }
                    }
                }
            }
        }

        self.dimensions = dimensions_new;
        self.expand_if_needed(&expand_x, &expand_y, &expand_z);
    }

    fn expand_if_needed(
        &mut self,
        expand_x: &(bool, bool),
        expand_y: &(bool, bool),
        expand_z: &(bool, bool),
    ) {
        self.expand_x_if_needed(expand_x);
        self.expand_y_if_needed(expand_y);
        self.expand_z_if_needed(expand_z);
    }

    fn expand_x_if_needed(&mut self, expand_x: &(bool, bool)) {
        if expand_x.0 {
            for index_z in 0..self.coordinates.z {
                for index_y in 0..self.coordinates.y {
                    let mut dimension_x_new: Vec<bool> = Vec::with_capacity(self.coordinates.x + 1);
                    dimension_x_new.push(false);
                    dimension_x_new.append(&mut self.dimensions[index_z][index_y]);
                    self.dimensions[index_z][index_y] = dimension_x_new;
                }
            }
    
            self.coordinates.x += 1;
        }

        if expand_x.1 {
            for index_z in 0..self.coordinates.z {
                for index_y in 0..self.coordinates.y {
                    self.dimensions[index_z][index_y].push(false);
                }
            }
            self.coordinates.x += 1;
        }
    }

    fn expand_y_if_needed(&mut self, expand_y: &(bool, bool)) {
        if expand_y.0 {
            for index_z in 0..self.coordinates.z {
                let mut dimension_xy_new: Vec<Vec<bool>> =
                    Vec::with_capacity(self.coordinates.y + 1);
                dimension_xy_new.push(Space::create_x_dimensions(self.coordinates.x));
                dimension_xy_new.append(&mut self.dimensions[index_z]);
                self.dimensions[index_z] = dimension_xy_new;
            }
            self.coordinates.y += 1;
        }

        if expand_y.1 {
            for index_z in 0..self.coordinates.z {
                self.dimensions[index_z].push(Space::create_x_dimensions(self.coordinates.x));
            }
            self.coordinates.y += 1;
        }

        debug_assert_eq!(self.coordinates.y, self.dimensions[0].len());
    }

    fn expand_z_if_needed(&mut self, expand_z: &(bool, bool)) {
        if expand_z.0 {
            let mut dimensions_new: Vec<Vec<Vec<bool>>> =
                Vec::with_capacity(self.coordinates.z + 1);
            dimensions_new.push(Space::create_xy_dimensions(
                self.coordinates.x,
                self.coordinates.y,
            ));
            dimensions_new.append(&mut self.dimensions);
            self.dimensions = dimensions_new;
            self.coordinates.z += 1;
        }

        if expand_z.1 {
            self.dimensions.push(Space::create_xy_dimensions(
                self.coordinates.x,
                self.coordinates.y,
            ));
            self.coordinates.z += 1;
        }

        debug_assert_eq!(self.coordinates.z, self.dimensions.len());
    }

    fn count_active_surrounding(&self, index_x: usize, index_y: usize, index_z: usize) -> usize {
        const SURROUNDING: [(i32, i32, i32); 26] = [
            (-1, -1, -1),
            (0, -1, -1),
            (1, -1, -1),
            (-1, 0, -1),
            (0, 0, -1),
            (1, 0, -1),
            (-1, 1, -1),
            (0, 1, -1),
            (1, 1, -1),
            (-1, -1, 0),
            (0, -1, 0),
            (1, -1, 0),
            (-1, 0, 0),
            (1, 0, 0),
            (-1, 1, 0),
            (0, 1, 0),
            (1, 1, 0),
            (-1, -1, 1),
            (0, -1, 1),
            (1, -1, 1),
            (-1, 0, 1),
            (0, 0, 1),
            (1, 0, 1),
            (-1, 1, 1),
            (0, 1, 1),
            (1, 1, 1),
        ];

        let location = Coordinates::from(index_x, index_y, index_z);
        let mut count: usize = 0;
        for surrounding in SURROUNDING.iter() {
            let location_to_check = location.sub(&surrounding, &self.coordinates);
            match location_to_check {
                SubResult::Overflow => {
                    // println!("{:?} - {:?} = out of map", location, surrounding);
                },
                SubResult::Result(location_to_check) => {
                    // println!("{:?} - {:?} = {:?}", location, surrounding, location_to_check);
                    debug_assert_ne!(location_to_check, location);
                    if self.at(&location_to_check) {
                        count += 1;
                    }
                }
            }
        }

        return count;
    }

    fn at(&self, coordinates: &Coordinates) -> bool {
        self.dimensions[coordinates.z][coordinates.y][coordinates.x]
    }

    pub fn count_actives(&self) -> u32 {
        let mut sum: u32 = 0;
        for dim_xy in self.dimensions.iter() {
            for dim_x in dim_xy.iter() {
                for value in dim_x.iter() {
                    if *value {
                        sum += 1;
                    }
                }
            }
        }
        return sum;
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{:?}", self.coordinates);
        for index_z in 0..self.coordinates.z {
            println!("z={}", index_z);
            let dim_xy = &self.dimensions[index_z];
            for index_y in 0..self.coordinates.y {
                let dim_x = &dim_xy[index_y];
                let mut line = String::new();
                for item in dim_x.iter() {
                    if *item {
                        line.push('#');
                    } else {
                        line.push('.');
                    }
                }

                println!("{}", line);
            }
            println!();
        }
    }

    pub fn from_input(input: &str) -> Space {
        let mut input_lines = input.lines();
        let max_x = input_lines.next().unwrap().len() + 2;
        let max_y = input_lines.count() + 3;
        let mut space = Space::create(max_x, max_y, 3);

        for (index_y, line) in input.lines().enumerate() {
            if line.len() + 2 != max_x {
                panic!(format!(
                    "Unexpected input dimension, found {}, expected: {}",
                    line.len(),
                    max_x
                ));
            }
            let line_space = &mut space.dimensions[1][index_y + 1];
            for (index_x, char) in line.chars().enumerate() {
                match char {
                    '.' => {}
                    '#' => {
                        line_space[index_x + 1] = true;
                    }
                    _ => panic!("Unexpected input char"),
                }
            }
        }

        return space;
    }

    fn create_x_dimensions(x: usize) -> Vec<bool> {
        vec![false; x]
    }

    fn create_xy_dimensions(x: usize, y: usize) -> Vec<Vec<bool>> {
        vec![Space::create_x_dimensions(x); y]
    }

    fn create_dimensions(x: usize, y: usize, z: usize) -> Vec<Vec<Vec<bool>>> {
        vec![Space::create_xy_dimensions(x, y); z]
    }

    fn create_dimensions_coordinates(coordinates: &Coordinates) -> Vec<Vec<Vec<bool>>> {
        Space::create_dimensions(coordinates.x, coordinates.y, coordinates.z)
    }

    fn create(x: usize, y: usize, z: usize) -> Self {
        let dimensions = Self::create_dimensions(x, y, z);
        Self {
            dimensions: dimensions,
            coordinates: Coordinates { x: x, y: y, z: z },
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
    fn example_parse_dimensions() {
        let input = get_example_input();
        let space = Space::from_input(input);

        assert_eq!(Coordinates::from(5, 5, 3), space.coordinates);
    }

    #[test]
    fn example2_parse_dimensions() {
        let space = Space::from_input("..#\n##.");

        assert_eq!(Coordinates::from(5, 4, 3), space.coordinates);
    }

    #[test]
    fn example_parsed_active_count() {
        let input = get_example_input();
        let space = Space::from_input(input);

        let result = space.count_actives();

        assert_eq!(5, result);
    }

    #[test]
    fn example_parsed_count_active_surrounding() {
        let input = get_example_input();
        let space = Space::from_input(input);

        let result = space.count_active_surrounding(3, 1, 1);

        assert_eq!(2, result);
    }

    #[test]
    fn example_parsed_count_active_surrounding_upper_edge() {
        let input = get_example_input();
        let space = Space::from_input(input);

        let result = space.count_active_surrounding(2, 0, 1);

        assert_eq!(1, result);
    }

    #[test]
    fn example_parsed_count_active_surrounding_bottom_right_corner() {
        let input = get_example_input();
        let space = Space::from_input(input);

        let result = space.count_active_surrounding(0, 4, 0);

        assert_eq!(1, result);
    }

    #[test]
    fn example_parsed_count_active_surrounding_active_place() {
        let input = get_example_input();
        let space = Space::from_input(input);

        let result = space.count_active_surrounding(2, 1, 1);

        assert_eq!(1, result);
    }

    #[test]
    fn example_parsed_simlate_cycle_count_actives() {
        let input = get_example_input();
        let mut space = Space::from_input(input);

        space.simlate_cycle();
        let result = space.count_actives();

        assert_eq!(11, result);
    }

    #[test]
    fn example_parsed_simlate_cycle_grew() {
        let input = get_example_input();
        let mut space = Space::from_input(input);

        space.simlate_cycle();

        assert_eq!(Coordinates::from(5, 6, 5), space.coordinates);
    }

    #[test]
    fn example_parsed_simlate_cycle2_count_actives() {
        let input = get_example_input();
        let mut space = Space::from_input(input);

        space.simlate_cycle();
        space.simlate_cycle();
        let result = space.count_actives();

        assert_eq!(21, result);
    }

    #[test]
    fn example_parsed_simlate_cycle2_grew() {
        let input = get_example_input();
        let mut space = Space::from_input(input);

        space.simlate_cycle();
        space.simlate_cycle();

        assert_eq!(Coordinates::from(7, 7, 7), space.coordinates);
    }
}
