use std::collections::{HashSet, HashMap, hash_map::Entry};

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = count_surface(input);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = count_surface_without_bubbles(input);
    return result.to_string();
}

fn count_surface_without_bubbles(input: &str) -> usize {
    let cubes = parse_input(input);
    let mut plane = Plane::new();
    for cube in cubes.iter() {
        plane.add_cube(cube);
    }
    let area = plane.area;

    let mut limits = plane.limits.clone();
    limits.expand();
    let mut search = Vec::new();
    search.push(Cube::from(limits.min_x, limits.min_y, limits.min_z));

    while let Some (cube) = search.pop() {
        if !limits.is_within(&cube) {
            continue;
        }
        if !plane.try_add_cube(&cube) {
            continue;
        }

        search.push(cube.get_left());
        search.push(cube.get_right());
        search.push(cube.get_above());
        search.push(cube.get_below());
        search.push(cube.get_behind());
        search.push(cube.get_in_front());
    }

    let x_length = limits.max_x - limits.min_x + 1;
    let y_length = limits.max_y - limits.min_y + 1;
    let z_length = limits.max_z - limits.min_z + 1;

    let area_xy = x_length * y_length * 2;
    let area_xz = x_length * z_length * 2;
    let area_yz = y_length * z_length * 2;

    let area_bubble = plane.area - area_xy as usize - area_xz as usize - area_yz as usize;
    return area - area_bubble;
}

fn count_surface(input: &str) -> usize {
    let cubes = parse_input(input);
    let mut plane = Plane::new();
    for cube in cubes.iter() {
        plane.add_cube(cube);
    }

    return plane.area;
}

#[derive(Clone, Copy)]
struct PlaneLimits {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

impl PlaneLimits {
    fn is_within(&self, cube: &Cube) -> bool {
        if cube.x > self.max_x || cube.x < self.min_x {
            return false;
        }
        if cube.y > self.max_y || cube.y < self.min_y {
            return false;
        }
        if cube.z > self.max_z || cube.z < self.min_z {
            return false;
        }
        return true;
    }

    fn update(&mut self, cube: &Cube) {
        self.min_x = self.min_x.min(cube.x);
        self.max_x = self.max_x.max(cube.x);
        self.min_y = self.min_y.min(cube.y);
        self.max_y = self.max_y.max(cube.y);
        self.min_z = self.min_z.min(cube.z);
        self.max_z = self.max_z.max(cube.z);
    }

    fn expand(&mut self) {
        self.min_x -= 1;
        self.max_x += 1;
        self.min_y -= 1;
        self.max_y += 1;
        self.min_z -= 1;
        self.max_z += 1;
    }

    fn set(&mut self, cube: &Cube) {
        self.min_x = cube.x;
        self.max_x = cube.x;
        self.min_y = cube.y;
        self.max_y = cube.y;
        self.min_z = cube.z;
        self.max_z = cube.z;
    }

    fn new() -> PlaneLimits {
        PlaneLimits {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            min_z: 0,
            max_z: 0,
        }
    }
}

struct Plane {
    surface: HashMap<i32, HashMap<i32, HashSet<i32>>>,
    area: usize,
    limits: PlaneLimits,
}

impl Plane {
    fn add_cube (&mut self, cube: &Cube) {
        if !self.try_add_cube(cube) {
            panic!()
        }
    }

    fn try_add_cube(&mut self, cube: &Cube) -> bool {
        if self.surface.len() == 0 {
            let mut z_column = HashSet::new();
            z_column.insert(cube.z);
            let mut y_column = HashMap::new();
            y_column.insert(cube.y, z_column);
            self.surface.insert(cube.x, y_column);
            self.limits.set(cube);
            self.recalculate_area(cube);
            return true;
        }
        match self.surface.entry(cube.x) {
            Entry::Occupied(mut entry_x) => match entry_x.get_mut().entry(cube.y) {
                Entry::Occupied(mut entry_y) => {
                    let z_column = entry_y.get_mut();
                    if z_column.contains(&cube.z) {
                        return false;
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
        self.limits.update(cube);
        self.recalculate_area(cube);
        return true;
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
            limits: PlaneLimits::new(),
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

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = count_surface_without_bubbles(input);

        assert_eq!(result, 58);
    }

    #[test]
    fn part2_input() {
        let result = get_solution_part2();

        assert_eq!(result, "2582");
    }
}
