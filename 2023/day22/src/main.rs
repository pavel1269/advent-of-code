use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

fn main() {
    let input = get_input();
    let result_part1 = part1(&input);
    println!("Part1: {}", result_part1);
    let result_part2 = part2(&input);
    println!("Part2: {}", result_part2);
}

fn part1(input: &str) -> usize {
    let mut bricks = parse_input(input);
    land_bricks(&mut bricks);

    let mut supports = HashSet::new();
    for (index, brick) in bricks.iter().enumerate() {
        if brick.is_on_groud() {
            continue;
        }
        let support = supporting_bricks(index, &bricks);
        if support.len() == 1 {
            supports.insert(support.iter().next().copied().unwrap());
        }
    }

    let result = bricks.len() - supports.len();
    return result;
}

fn part2(input: &str) -> usize {
    let mut bricks = parse_input(input);
    land_bricks(&mut bricks);

    let mut supports = HashMap::with_capacity(bricks.len());
    let mut holds: HashMap<usize, HashSet<usize>> = HashMap::with_capacity(bricks.len());
    for (index, brick) in bricks.iter().enumerate() {
        if brick.is_on_groud() {
            continue;
        }
        let support = supporting_bricks(index, &bricks);
        supports.insert(index, support.clone());
        support.into_iter().for_each(|holder| {
            holds.entry(holder).or_default().insert(index);
        });
    }

    let mut fall = 0;
    for index in bricks.iter().enumerate().map(|(index, _)| index) {
        let affected = holds.get(&index);
        if affected.is_none() {
            continue;
        }
        let mut affected = affected.unwrap().clone();
        affected.insert(index);

        let mut removed = HashSet::from([index]);
        let mut removed_last_count = 0;
        while removed.len() != removed_last_count {
            removed_last_count = removed.len();
            supports
                .iter()
                .filter(|(support_index, supports)| {
                    affected.contains(&support_index)
                        && supports
                            .iter()
                            .all(|support_index| removed.contains(support_index))
                })
                .map(|(&index, _)| index)
                .collect::<HashSet<_>>()
                .into_iter()
                .for_each(|index| {
                    removed.insert(index);
                });
            holds
                .iter()
                .filter(|(hold_index, _)| removed.contains(&hold_index))
                .flat_map(|(_, holds)| holds)
                .for_each(|&affectee| {
                    affected.insert(affectee);
                });
        }

        fall += removed.len() - 1;
    }
    return fall;
}

fn land_bricks(bricks: &mut Vec<Brick>) {
    let mut bricks_landed = bricks
        .iter()
        .enumerate()
        .map(|(index, brick)| (index, brick.is_on_groud()))
        .collect::<HashMap<_, _>>();
    while bricks_landed.values().any(|&landed| !landed) {
        let unlanded_bricks = bricks_landed
            .iter()
            .filter(|(_, &landed)| !landed)
            .map(|(&index, _)| index)
            .collect::<Vec<_>>();
        for index in unlanded_bricks.into_iter() {
            let collided_with = supporting_bricks(index, bricks);
            if collided_with.len() > 0 {
                if collided_with.iter().any(|index| bricks_landed[index]) {
                    *bricks_landed.get_mut(&index).unwrap() = true;
                }
            } else {
                let brick = bricks.get_mut(index).unwrap();
                brick.move_down();
                if brick.is_on_groud() {
                    *bricks_landed.get_mut(&index).unwrap() = true;
                }
            }
        }
    }
}

fn supporting_bricks(index: usize, bricks: &Vec<Brick>) -> HashSet<usize> {
    let brick = bricks.get(index).unwrap();
    let mut iterators = brick.generate_iterators();
    let mut collided_with = HashSet::new();
    assert_eq!(iterators.len(), 3);
    for x in iterators.get_mut(0).unwrap().clone() {
        for y in iterators.get_mut(1).unwrap().clone() {
            let point = Point {
                coords: [x, y, iterators.get(2).unwrap().start() - 1],
            };
            for (index_collided, brick_other) in bricks.iter().enumerate() {
                if index == index_collided {
                    continue;
                }
                if brick_other.is_inside(&point) {
                    collided_with.insert(index_collided);
                }
            }
        }
    }
    return collided_with;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Brick {
    start: Point,
    end: Point,
}

impl Brick {
    fn is_inside(&self, point: &Point) -> bool {
        let iters = self.generate_iterators();
        for (index, iter) in iters.into_iter().enumerate() {
            if iter.start() > &point.coords[index] || iter.end() < &point.coords[index] {
                return false;
            }
        }
        return true;
    }

    fn generate_iterators(&self) -> Vec<RangeInclusive<usize>> {
        let start = &self.start.coords;
        let end = &self.end.coords;

        let result = start
            .iter()
            .enumerate()
            .map(|(index, &coord)| {
                let other_coord = end[index];
                return coord.min(other_coord)..=coord.max(other_coord);
            })
            .collect::<Vec<_>>();
        return result;
    }

    fn move_down(&mut self) {
        self.start.coords[2] -= 1;
        self.end.coords[2] -= 1;
    }

    fn is_on_groud(&self) -> bool {
        self.start.coords[2] == 1 || self.end.coords[2] == 1
    }

    fn from(str: &str) -> Self {
        let points: Vec<_> = str.split('~').collect();
        assert_eq!(points.len(), 2);
        let start = Point::from(points[0]);
        let end = Point::from(points[1]);
        start
            .coords
            .iter()
            .zip(end.coords.iter())
            .for_each(|(a, b)| assert!(a <= b));
        let result = Self { start, end };
        return result;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    coords: [usize; 3],
}

impl Point {
    fn from(str: &str) -> Self {
        let coords = str
            .split(',')
            .map(|str| str.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let result = Self { coords };
        return result;
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    input.lines().map(|line| Brick::from(line)).collect()
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case([0,1,0], false)]
    #[test_case([1,0,0], true)]
    #[test_case([0,0,0], true)]
    #[test_case([0,0,2], false)]
    #[test_case([0,1,1], false)]
    #[test_case([0,2,0], false)]
    #[test_case([2,0,0], true)]
    fn brick_is_inside(coords: [usize; 3], expect: bool) {
        let brick = Brick {
            start: Point { coords: [0, 0, 0] },
            end: Point { coords: [2, 0, 0] },
        };
        let test_point = Point { coords };
        let result = brick.is_inside(&test_point);
        assert_eq!(result, expect);
    }

    fn get_example_input() -> &'static str {
        include_str!("./example.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = part1(&input);
        assert_eq!(result, 5);
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = part2(&input);
        assert_eq!(result, 7);
    }
}
