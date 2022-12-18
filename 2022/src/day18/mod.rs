use std::collections::{HashSet, HashMap, hash_map::Entry};


pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = count_surface(input);
    return result.to_string();
}


fn count_surface(input: &str) -> usize {
    let cubes = parse_input(input);
    let mut plane = Plane::new();
    for cube in cubes.iter() {
        plane.add_cube(cube);
    }

    return plane.area;
}

struct Plane {
    surface: HashMap<i32, HashMap<i32, HashSet<i32>>>,
    area: usize,
}

impl Plane {
    fn add_cube (&mut self, cube: &Cube) {
        match self.surface.entry(cube.x) {
            Entry::Occupied(mut entry_x) => match entry_x.get_mut().entry(cube.y) {
                Entry::Occupied(mut entry_y) => {
                    let z_column = entry_y.get_mut();
                    if z_column.contains(&cube.z) {
                        panic!()
                    }
                    z_column.insert(cube.z);
                },
                Entry::Vacant(entry_y) => {
                    let mut z_column = HashSet::new();
                    z_column.insert(cube.z);
                    entry_y.insert(z_column);
                },
            },
            Entry::Vacant(entry_x) => {
                let mut z_column = HashSet::new();
                z_column.insert(cube.z);
                let mut y_column = HashMap::new();
                y_column.insert(cube.y, z_column);
                entry_x.insert(y_column);
            },
        };
        self.recalculate_area(cube);
    }

    fn recalculate_area(&mut self, cube: &Cube) {
        let mut covered_sides = 0;
        let cube_new = cube.get_left();
        if self.is_occupied(&cube_new) {
            covered_sides += 1;
        }
        let cube_new = cube.get_right();
        if self.is_occupied(&cube_new) {
            covered_sides += 1;
        }
        let cube_new = cube.get_below();
        if self.is_occupied(&cube_new) {
            covered_sides += 1;
        }
        let cube_new = cube.get_above();
        if self.is_occupied(&cube_new) {
            covered_sides += 1;
        }
        let cube_new = cube.get_behind();
        if self.is_occupied(&cube_new) {
            covered_sides += 1;
        }
        let cube_new = cube.get_in_front();
        if self.is_occupied(&cube_new) {
            covered_sides += 1;
        }

        self.area = self.area - covered_sides + 6 - covered_sides;
    }

    fn is_occupied(&self, cube: &Cube) -> bool {
        match self.surface.get(&cube.x) {
            None => false,
            Some(entry_x) => match entry_x.get(&cube.y) {
                None => false,
                Some(entry_y) => entry_y.contains(&cube.z),
            },
        }
    }

    fn new() -> Plane {
        Plane {
            surface: HashMap::new(),
            area: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn from(x: i32, y: i32, z: i32) -> Cube {
        Cube {
            x,
            y,
            z,
        }
    }

    fn get_left(&self) -> Cube {
        let mut copy = *self;
        copy.x -= 1;
        return copy;
    }
    fn get_right(&self) -> Cube {
        let mut copy = *self;
        copy.x += 1;
        return copy;
    }
    fn get_above(&self) -> Cube {
        let mut copy = *self;
        copy.y -= 1;
        return copy;
    }
    fn get_below(&self) -> Cube {
        let mut copy = *self;
        copy.y += 1;
        return copy;
    }
    fn get_behind(&self) -> Cube {
        let mut copy = *self;
        copy.z -= 1;
        return copy;
    }
    fn get_in_front(&self) -> Cube {
        let mut copy = *self;
        copy.z += 1;
        return copy;
    }
}

fn parse_input(input: &str) -> Vec<Cube> {
    let mut cubes = Vec::new();
    for row in input.lines() {
        let coordinates: Vec<i32> = row.split(",").map(|c| c.parse().unwrap()).collect();
        if coordinates.len() != 3 {
            panic!()
        }
        let cube = Cube::from(coordinates[0], coordinates[1], coordinates[2]);
        cubes.push(cube);
    }
    return cubes;
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = count_surface(input);

        assert_eq!(result, 64);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "4628");
    }
}
