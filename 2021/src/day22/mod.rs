use std::collections::HashSet;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = count_cubes_initialization(input);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = count_cubes_restart(input);
    return result;
}

#[derive(Clone, Debug)]
struct Segment {
    segment: Vec<((i64, i64), Option<Segment>)>,
}

impl Segment {
    pub fn edit(&mut self, is_on: bool, sizes: &Vec<(i64, i64)>) {
        let mut handle_remaining = true;
        let mut add_later = Vec::new();
        let mut remove_later = Vec::new();
        let (mut size0, size1) = sizes.first().copied().unwrap();

        for (index, ((coord0, coord1), child)) in self.segment.iter_mut().enumerate() {
            if !handle_remaining {
                // println!("!handle_remaining");
                break;
            }
            if *coord1 < size0 {
                // println!("coord1 < size0");
                continue;
            }
            if *coord0 > size1 {
                // println!("coord0 > size1");
                break;
            }
            // println!("working on [{},{}] against [{},{}]", size0, size1, coord0, coord1);

            if sizes.len() > 1 {
                let child = child.as_mut().unwrap();
                let coords = sizes.iter().skip(1).copied().collect();
                if *coord0 == size0 {
                    if *coord1 == size1 {
                        // Size   0    |-----|     1
                        // Coords 0    |-----|     1
                        child.edit(is_on, &coords);
                        if child.is_empty() {
                            remove_later.push(index);
                        }
                        handle_remaining = false;
                    } else if *coord1 > size1 {
                        // Size   0    |-----|     1
                        // Coords 0    |--------|  1
                        add_later.push(((size1 + 1, *coord1), Some(child.clone())));
                        child.edit(is_on, &coords);
                        if !child.is_empty() {
                            add_later.push(((size0, size1), Some(child.clone())));
                        }
                        remove_later.push(index);
                        handle_remaining = false;
                    } else {
                        assert!(*coord1 < size1);
                        // Size   0    |-----|     1
                        // Coords 0    |---|       1
                        size0 = *coord1 + 1;
                        child.edit(is_on, &coords);
                        if child.is_empty() {
                            remove_later.push(index);
                        }
                    }
                } else if *coord1 == size1 {
                    if *coord0 < size0 {
                        // Size   0    |-----|     1
                        // Coords 0  |-------|     1
                        add_later.push(((*coord0, size0 - 1), Some(child.clone())));
                        child.edit(is_on, &coords);
                        if !child.is_empty() {
                            add_later.push(((size0, size1), Some(child.clone())));
                        }
                        remove_later.push(index);
                    } else {
                        assert!(*coord0 > size0);
                        // Size   0  |-------|     1
                        // Coords 0    |-----|     1
                        child.edit(is_on, &coords);
                        if child.is_empty() {
                            remove_later.push(index);
                        } else if is_on {
                            let mut child = Segment::new();
                            child.edit(true, &coords);
                            add_later.push(((size0, *coord0 - 1), Some(child)));
                        }
                    }
                    handle_remaining = false;
                } else if *coord0 < size0 {
                    if *coord1 < size1 {
                        // Size   0    |-----|     1
                        // Coords 0  |-----|       1
                        add_later.push(((*coord0, size0 - 1), Some(child.clone())));
                        child.edit(is_on, &coords);
                        if !child.is_empty() {
                            add_later.push(((size0, *coord1), Some(child.clone())));
                        }
                        size0 = *coord1 + 1;
                    } else {
                        assert!(*coord1 > size1);
                        // Size   0    |-----|     1
                        // Coords 0  |---------|   1
                        add_later.push(((*coord0, size0 - 1), Some(child.clone())));
                        add_later.push(((size1 + 1, *coord1), Some(child.clone())));
                        child.edit(is_on, &coords);
                        if !child.is_empty() {
                            add_later.push(((size0, size1), Some(child.clone())));
                        }
                        handle_remaining = false;
                    }
                    remove_later.push(index);
                } else {
                    assert!(*coord0 > size0);
                    if *coord1 < size1 {
                        // Size   0  |---------|   1
                        // Coords 0    |-----|     1
                        child.edit(is_on, &coords);
                        if child.is_empty() {
                            size0 = *coord1 + 1;
                            remove_later.push(index);
                        } else if is_on {
                            let mut child = Segment::new();
                            child.edit(true, &coords);
                            add_later.push(((size0, *coord0 - 1), Some(child)));
                            size0 = *coord1 + 1;
                        }
                    } else {
                        assert!(*coord1 > size1);
                        // Size   0  |-------|     1
                        // Coords 0    |-------|   1
                        add_later.push(((size1 + 1, *coord1), Some(child.clone())));
                        child.edit(is_on, &coords);
                        if !child.is_empty() {
                            add_later.push(((*coord0, size1), Some(child.clone())));
                            if is_on {
                                let mut child = Segment::new();
                                child.edit(true, &coords);
                                add_later.push(((size0, *coord0 - 1), Some(child)));
                            }
                        }
                        remove_later.push(index);
                        handle_remaining = false;
                    }
                }
            } else {
                assert!(sizes.len() == 1);
                if *coord0 == size0 {
                    if *coord1 == size1 {
                        if !is_on {
                            remove_later.push(index);
                        }
                        handle_remaining = false;
                    } else if *coord1 > size1 {
                        if !is_on {
                            add_later.push(((size1 + 1, *coord1), None));
                            remove_later.push(index);
                        }
                        handle_remaining = false;
                    } else {
                        size0 = *coord1 + 1;
                        if !is_on {
                            remove_later.push(index);
                        }
                    }
                } else if *coord1 == size1 {
                    if *coord0 < size0 {
                        if !is_on {
                            add_later.push(((*coord0, size0 - 1), None));
                            remove_later.push(index);
                        }
                    } else {
                        assert!(*coord0 > size0);
                        if is_on {
                            add_later.push(((size0, *coord0 - 1), None));
                        } else {
                            remove_later.push(index);
                        }
                    }
                    handle_remaining = false;
                } else if *coord0 < size0 {
                    if *coord1 < size1 {
                        if is_on {
                            size0 = *coord1 + 1;
                        } else {
                            add_later.push(((*coord0, size0 - 1), None));
                            size0 = *coord1 + 1;
                            remove_later.push(index);
                        }
                    } else {
                        assert!(*coord1 > size1);
                        if !is_on {
                            add_later.push(((*coord0, size0 - 1), None));
                            add_later.push(((size1 + 1, *coord1), None));
                            remove_later.push(index);
                        }
                        handle_remaining = false;
                    }
                } else {
                    assert!(*coord0 > size0);
                    if *coord1 < size1 {
                        if is_on {
                            add_later.push(((size0, *coord0 - 1), None));
                            size0 = *coord1 + 1;
                        } else {
                            size0 = *coord1 + 1;
                            remove_later.push(index);
                        }
                    } else {
                        assert!(*coord1 > size1);
                        if is_on {
                            add_later.push(((size0, *coord0 - 1), None));
                        } else {
                            add_later.push(((size1 + 1, *coord1), None));
                            remove_later.push(index);
                        }
                        handle_remaining = false;
                    }
                }
            }
        }

        for index in remove_later.iter().rev() {
            self.segment.remove(*index);
        }
        for segment in add_later {
            self.segment.push(segment);
        }
        if handle_remaining && is_on && size1 >= size0 {
            let child = if sizes.len() > 1 {
                let mut child = Segment::new();
                let child_coords = sizes.iter().skip(1).copied().collect::<Vec<_>>();
                child.edit(true, &child_coords);
                Some(child)
            } else {
                None
            };
            self.segment.push(((size0, size1), child));
        }

        self.segment.sort_by(|a, b| a.0.0.cmp(&b.0.0));
        // println!("segment: {:?}", self);
        // println!();
    }

    pub fn size(&self) -> i64 {
        let mut size = 0;
        for ((coord0, coord1), child) in self.segment.iter() {
            let distance = coord1 - coord0 + 1;
            assert!(distance > 0);
            if let Some(child) = child {
                size += distance * child.size();
            } else {
                size += distance;
            }
        }
        return size;
    }

    pub fn is_empty(&self) -> bool {
        self.segment.len() == 0
    }

    pub fn new() -> Segment {
        Segment {
            segment: Vec::new(),
        }
    }
}

fn count_cubes_restart(input: &str) -> i64 {
    let steps = parse_input(input);
    let mut segment = Segment::new();

    for (is_on, x, y, z) in steps.iter() {
        segment.edit(*is_on, &vec![*x, *y, *z]);
    }

    // println!("result: {:?}", segment);
    return segment.size();
}

fn count_cubes_initialization(input: &str) -> i64 {
    const BOUNDARY_BOTTOM: i64 = -50;
    const BOUNDARY_UPPER: i64 = 51;

    let steps = parse_input(input);
    let mut cubes: HashSet<(i64, i64, i64)> = HashSet::new();

    for (is_on, x, y, z) in steps.iter() {
        for x in x.0.max(BOUNDARY_BOTTOM)..(x.1 + 1).min(BOUNDARY_UPPER) {
            for y in y.0.max(BOUNDARY_BOTTOM)..(y.1 + 1).min(BOUNDARY_UPPER) {
                for z in z.0.max(BOUNDARY_BOTTOM)..(z.1 + 1).min(BOUNDARY_UPPER) {
                    if *is_on {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    return cubes.len() as i64;
}

fn parse_input(input: &str) -> Vec<(bool, (i64, i64), (i64, i64), (i64, i64))> {
    use regex::Regex;

    lazy_static::lazy_static! {
        static ref REGEX: Regex = Regex::new("^(on|off) x=(-?\\d+)..(-?\\d+),y=(-?\\d+)..(-?\\d+),z=(-?\\d+)..(-?\\d+)$").unwrap();
    }

    return input
        .lines()
        .map(|line| {
            let captures = REGEX.captures(line).unwrap();
            let is_on = &captures[1] == "on";
            let x = (captures[2].parse().unwrap(), captures[3].parse().unwrap());
            let y = (captures[4].parse().unwrap(), captures[5].parse().unwrap());
            let z = (captures[6].parse().unwrap(), captures[7].parse().unwrap());
            return (is_on, x, y, z);
        })
        .collect();
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input_1() -> &'static str {
        return "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
    }

    fn get_example_input_2() -> &'static str {
        return "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";
    }

    #[test]
    fn count_cubes_initialization_example1_test() {
        let input = get_example_input_1();
        let result = count_cubes_initialization(input);
        assert_eq!(39, result);
    }

    #[test]
    fn count_cubes_initialization_example2_test() {
        let input = get_example_input_2();
        let result = count_cubes_initialization(input);
        assert_eq!(590784, result);
    }

    #[test]
    fn part1_test() {
        let result = get_solution_part1();
        assert_eq!(588200, result);
    }

    macro_rules! count_cubes_restart_test {
        ($name: ident, $input: expr, $result: expr) => {
            #[test]
            fn $name() {
                let result = count_cubes_restart($input);
                assert_eq!($result, result);
            }
        };
    }

    count_cubes_restart_test!(count_cubes_restart_simple_test_1, "on x=0..0,y=0..0,z=0..0", 1);
    count_cubes_restart_test!(count_cubes_restart_simple_test_2, "on x=0..2,y=0..2,z=0..2", 27);

    count_cubes_restart_test!(count_cubes_restart_simple_test_eq_01, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=0..2,z=0..2", 27);
    count_cubes_restart_test!(count_cubes_restart_simple_test_eq_02, "on x=0..2,y=0..2,z=0..2
off x=0..2,y=0..2,z=0..2", 27 - 27);
    count_cubes_restart_test!(count_cubes_restart_simple_test_off_01, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=2..4,z=0..2", 27 + 18);
    count_cubes_restart_test!(count_cubes_restart_simple_test_off_02, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=-1..0,z=0..2", 27 + 9);
    count_cubes_restart_test!(count_cubes_restart_simple_test_off_03, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=0..2,z=2..4", 27 + 18);

    count_cubes_restart_test!(count_cubes_restart_simple_test_sub1_01, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=0..1,z=0..2", 27);
    count_cubes_restart_test!(count_cubes_restart_simple_test_sub1_02, "on x=0..2,y=0..2,z=0..2
off x=0..2,y=0..1,z=0..2", 27 - 18);
    count_cubes_restart_test!(count_cubes_restart_simple_test_sub1_03, "on x=0..2,y=0..2,z=0..2
off x=0..2,y=0..1,z=0..1", 27 - 12);
    count_cubes_restart_test!(count_cubes_restart_simple_test_sub1_04, "on x=0..2,y=0..2,z=0..2
off x=0..1,y=0..2,z=0..1", 27 - 12);

    count_cubes_restart_test!(count_cubes_restart_simple_test_sub2_01, "on x=0..2,y=0..2,z=0..2
off x=0..2,y=0..2,z=0..3", 0);
    count_cubes_restart_test!(count_cubes_restart_simple_test_sub2_02, "on x=0..2,y=0..2,z=0..2
off x=0..2,y=0..3,z=0..2", 0);
    count_cubes_restart_test!(count_cubes_restart_simple_test_sub2_03, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=0..3,z=0..2", 3 * 4 * 3);
    count_cubes_restart_test!(count_cubes_restart_simple_test_sub2_04, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=0..2,z=0..3", 3 * 3 * 4);

    count_cubes_restart_test!(count_cubes_restart_simple_test_sub3_01, "on x=0..2,y=0..2,z=0..2
off x=0..2,y=0..2,z=-1..2", 0);
    count_cubes_restart_test!(count_cubes_restart_simple_test_sub3_02, "on x=0..2,y=0..2,z=0..2
off x=0..2,y=-1..2,z=0..2", 0);
    count_cubes_restart_test!(count_cubes_restart_simple_test_sub3_03, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=-1..2,z=0..2", 3 * 4 * 3);
    count_cubes_restart_test!(count_cubes_restart_simple_test_sub3_04, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=0..2,z=-1..2", 3 * 3 * 4);

    count_cubes_restart_test!(count_cubes_restart_simple_test_right1_01, "on x=0..2,y=0..2,z=0..2
off x=0..2,y=1..3,z=0..2", 27 - 18);
    count_cubes_restart_test!(count_cubes_restart_simple_test_right1_02, "on x=0..2,y=0..2,z=0..2
off x=0..2,y=0..2,z=1..3", 27 - 18);
    count_cubes_restart_test!(count_cubes_restart_simple_test_right1_03, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=1..3,z=0..2", 3 * 4 * 3);
    count_cubes_restart_test!(count_cubes_restart_simple_test_right1_04, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=0..2,z=1..3", 3 * 3 * 4);

    count_cubes_restart_test!(count_cubes_restart_simple_test_right2_01, "on x=0..3,y=0..3,z=0..3
off x=0..3,y=1..2,z=0..3", 4 * 4 * 4 - 4 * 2 * 4);
    count_cubes_restart_test!(count_cubes_restart_simple_test_right2_02, "on x=0..3,y=0..3,z=0..3
off x=0..3,y=0..3,z=1..2", 4 * 4 * 4 - 4 * 4 * 2);
    count_cubes_restart_test!(count_cubes_restart_simple_test_right2_03, "on x=0..3,y=0..3,z=0..3
off x=0..3,y=1..2,z=1..2", 4 * 4 * 4 - 4 * 2 * 2);
    count_cubes_restart_test!(count_cubes_restart_simple_test_right2_04, "on x=0..3,y=0..3,z=0..3
on x=0..3,y=1..2,z=0..3", 4 * 4 * 4);
    count_cubes_restart_test!(count_cubes_restart_simple_test_right2_05, "on x=0..3,y=0..3,z=0..3
on x=0..3,y=0..3,z=1..2", 4 * 4 * 4);

    count_cubes_restart_test!(count_cubes_restart_simple_test_sub4_01, "on x=1..2,y=1..2,z=1..2
off x=1..2,y=1..2,z=0..3", 0);
    count_cubes_restart_test!(count_cubes_restart_simple_test_sub4_02, "on x=1..2,y=1..2,z=1..2
on x=1..2,y=1..2,z=0..3", 2 * 2 * 4);
    count_cubes_restart_test!(count_cubes_restart_simple_test_sub4_03, "on x=1..2,y=1..2,z=1..2
off x=1..2,y=0..3,z=1..2", 0);
    count_cubes_restart_test!(count_cubes_restart_simple_test_sub4_04, "on x=1..2,y=1..2,z=1..2
on x=1..2,y=0..3,z=1..2", 2 * 4 * 2);

    count_cubes_restart_test!(count_cubes_restart_simple_test_left1_01, "on x=0..2,y=0..2,z=0..2
off x=0..2,y=0..2,z=-1..1", 27 - 18);
    count_cubes_restart_test!(count_cubes_restart_simple_test_left1_02, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=0..2,z=-1..1", 36);
    count_cubes_restart_test!(count_cubes_restart_simple_test_left1_03, "on x=0..2,y=0..2,z=0..2
off x=0..2,y=-1..1,z=0..2", 27 - 18);
    count_cubes_restart_test!(count_cubes_restart_simple_test_left1_04, "on x=0..2,y=0..2,z=0..2
on x=0..2,y=-1..1,z=0..2", 36);

    count_cubes_restart_test!(count_cubes_restart_test1, "on x=10..10,y=10..11,z=12..12", 2);
    count_cubes_restart_test!(count_cubes_restart_test2, "on x=10..12,y=10..12,z=10..12
off x=9..11,y=9..11,z=9..11", 27 - 8);
    count_cubes_restart_test!(count_cubes_restart_test3, "on x=10..10,y=10..11,z=12..12
on x=10..10,y=12..12,z=10..12", 5);
    count_cubes_restart_test!(count_cubes_restart_test4, "on x=10..10,y=10..11,z=12..12
on x=10..10,y=12..12,z=10..12
on x=10..10,y=10..10,z=10..10", 6);
    count_cubes_restart_test!(count_cubes_restart_test5, "on x=10..12,y=10..12,z=10..12
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10", 27 - 8 + 1);
    count_cubes_restart_test!(count_cubes_restart_test6, "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13", 27 + 19);
    count_cubes_restart_test!(count_cubes_restart_test7, "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11", 27 + 19 - 8);

    count_cubes_restart_test!(count_cubes_restart_example1_test, get_example_input_1(), 39);

    fn get_example_input_3() -> &'static str {
        return include_str!("./example3.txt");
    }

    count_cubes_restart_test!(count_cubes_restart_example3_test, get_example_input_3(), 2758514936282235);

    #[test]
    fn part2_test() {
        let result = get_solution_part2();
        assert_eq!(1207167990362099, result);
    }
}
