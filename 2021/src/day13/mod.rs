use std::collections::HashSet;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = points_after_fold(input);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let paper = points_after_all_folds(input);
    print_paper(&paper);
    return -1;
}

fn print_paper(paper: &HashSet<(usize, usize)>) {
    let (width, height) = paper
        .iter()
        .copied()
        .reduce(|(x1, y1), (x2, y2)| (std::cmp::max(x1, x2), std::cmp::max(y1, y2)))
        .unwrap();

    for y in 0..height + 1 {
        for x in 0..width + 1 {
            if paper.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn points_after_all_folds(input: &str) -> HashSet<(usize, usize)> {
    let (folds, mut points) = parse_input(input);
    for (fold_x, distance) in folds {
        points = fold(&points, fold_x, distance);
    }
    return points;
}

fn points_after_fold(input: &str) -> i64 {
    let (folds, points) = parse_input(input);
    let (fold_x, distance) = *folds.first().unwrap();
    let folded_points = fold(&points, fold_x, distance);
    return folded_points.len() as i64;
}

fn fold(
    points: &HashSet<(usize, usize)>,
    fold_x: bool,
    distance: usize,
) -> HashSet<(usize, usize)> {
    let mut folded_points = HashSet::new();
    for (x, y) in points.iter().copied() {
        if fold_x {
            if x < distance {
                folded_points.insert((x, y));
            } else {
                folded_points.insert((distance - (x - distance), y));
            }
        } else {
            if y < distance {
                folded_points.insert((x, y));
            } else {
                folded_points.insert((x, distance - (y - distance)));
            }
        }
    }

    return folded_points;
}

fn parse_input(input: &str) -> (Vec<(bool, usize)>, HashSet<(usize, usize)>) {
    let mut fold = Vec::new();
    let mut points = HashSet::new();

    for line in input.lines() {
        if line.trim().len() <= 1 {
            continue;
        }

        if line.starts_with("fold along ") {
            let (first, second) = line.split_once('=').unwrap();
            let distance = second.parse::<usize>().unwrap();
            if first.ends_with("x") {
                fold.push((true, distance));
            } else {
                fold.push((false, distance));
            }
        } else {
            let (first, second) = line.split_once(',').unwrap();
            let x = first.parse().unwrap();
            let y = second.parse().unwrap();
            points.insert((x, y));
        }
    }

    return (fold, points);
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    }

    #[test]
    fn example_part1_correct_result() {
        let result = points_after_fold(get_example_input());

        assert_eq!(17, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(695, result);
    }

    #[test]
    fn example_part2_correct_result() {
        let paper = points_after_all_folds(get_example_input());
        print_paper(&paper);

        let result = paper.len();

        assert_eq!(16, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        // GJZGLUPJ
        assert_eq!(-1, result);
    }
}
