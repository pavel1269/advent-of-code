fn main() {
    let input = get_input();
    let result_part1 = part1(&input, 200000000000000.0, 400000000000000.0);
    println!("Part1: {}", result_part1);
}

fn part1(input: &str, min: f64, max: f64) -> usize {
    let hails = parse_input(input);
    let mut intercepts = 0;
    for (index, hail) in hails.iter().enumerate() {
        for hail_other in hails.iter().skip(index + 1) {
            if hail.intercept_2d(hail_other, &min, &max) {
                intercepts += 1;
            }
        }
    }
    return intercepts;
}

#[derive(Debug, Clone, Copy)]
struct Hail {
    position: Position,
    trajectory: Position,
}

impl Hail {
    fn intercept_2d(&self, other: &Hail, min: &f64, max: &f64) -> bool {
        // unknowns: t1 t2
        // eq1: x1 t1 + px1 = x2 t2 + px2
        // eq2: y1 t1 + py1 = y2 t2 + py2
        // eq1 -> x1 t1 = x2 t2 + px2 - px1
        // (x1 != 0) -> t1 = (x2 t2 + px2 - px1) / x1
        // // (x1 = 0) -> x2 t2 + px2 - px1 = 0
        // // (x1 = 0) -> x2 t2 = px1 - px2
        // // (x1 = 0 && x2 != 0) -> t2 = (px1 - px2) / x2
        // // (x1 = 0 && x2 = 0) -> px1 = px2
        // eq2 (x1 != 0) -> y1 (x2 t2 + px2 - px1) / x1 + py1 = y2 t2 + py2
        // (x1 != 0) -> y1 (x2 t2 + px2 - px1) / x1 = y2 t2 + py2 - py1
        // (x1 != 0) -> y1 (x2 t2 + px2 - px1) = y2 t2 x1 + py2 x1 - py1 x1
        // (x1 != 0) -> y1 x2 t2 + y1 px2 - y1 px1 = y2 t2 x1 + py2 x1 - py1 x1
        // (x1 != 0) -> y1 x2 t2 - y2 t2 x1 = py2 x1 - py1 x1 - y1 px2 + y1 px1
        // (x1 != 0) -> t2 (y1 x2 - y2 x1) = py2 x1 - py1 x1 - y1 px2 + y1 px1
        // (x1 != 0 && (y1 x2 - y2 x1) != 0) -> t2 = (py2 x1 - py1 x1 - y1 px2 + y1 px1) / (y1 x2 - y2 x1)
        // (x1 != 0 && (y1 x2 - y2 x1) = 0) -> py2 x1 - py1 x1 - y1 px2 + y1 px1 = 0
        
        let x1 = &self.trajectory.coords[0];
        let y1 = &self.trajectory.coords[1];
        let px1 = &self.position.coords[0];
        let py1 = &self.position.coords[1];
        let x2 = &other.trajectory.coords[0];
        let y2 = &other.trajectory.coords[1];
        let px2 = &other.position.coords[0];
        let py2 = &other.position.coords[1];

        let b = y1 * x2 - y2 * x1;
        let a = py2 * x1 - py1 * x1 - y1 * px2 + y1 * px1;

        if compare_floats_equal_enough(&b, &0.0) {
            let result = compare_floats_equal_enough(&a, &0.0);
            return result;
        }

        let t2 = a / b;
        let x = x2 * t2 + px2;
        let y = y2 * t2 + py2;
        let result = &x >= min && &x <= max && &y >= min && &y <= max;
        if !result {
            return false;
        }

        let future_1 = if x1 > &0.0 {
            &x > px1
        } else {
            px1 > &x
        };
        let future_2 = if x2 > &0.0 {
            &x > px2
        } else {
            px2 > &x
        };
        let result = future_1 && future_2;
        return result;
    }

    fn from(str: &str) -> Self {
        let input = str
            .split('@')
            .map(|str| Position::from(str))
            .collect::<Vec<_>>();
        let position = input[0];
        let trajectory = input[1];
        assert!(trajectory.coords.iter().all(|coord| !compare_floats_equal_enough(coord, &0.0)));
        let result = Self {
            position,
            trajectory,
        };
        return result;
    }
}

fn compare_floats_equal_enough(f1: &f64, f2: &f64) -> bool {
    if f1 == f2 {
        return true;
    }
    let diff = (f1 - f2).abs();
    let max = f1.abs().max(f2.abs());
    return diff < max / 1000.0;
}

#[derive(Debug, Clone, Copy)]
struct Position {
    coords: [f64; 3],
}

impl Position {
    fn from(str: &str) -> Self {
        let coords = str
            .split(',')
            .map(|str| str.trim().parse().unwrap())
            .collect::<Vec<f64>>()
            .try_into()
            .unwrap();
        let result = Self { coords };
        return result;
    }
}

fn parse_input(input: &str) -> Vec<Hail> {
    input.lines().map(|line| Hail::from(line)).collect()
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        include_str!("./example.txt")
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = part1(&input, 7.0, 27.0);
        assert_eq!(result, 2);
    }
}
