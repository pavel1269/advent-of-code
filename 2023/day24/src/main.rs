fn main() {
    let input = get_input();
    let result_part1 = part1(&input, 200000000000000.0, 400000000000000.0);
    println!("Part1: {}", result_part1);
    let result_part2 = part2(&input);
    println!("Part2: {}", result_part2);
    // 870379016024859
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

fn part2(input: &str) -> usize {
    // hail m
    // hail n in hails

    // unknowns: xm, ym, zm, pxm, pym, pzm, tn
    // xm tn + pxm = xn tn + pxn
    // ym tn + pym = yn tn + pyn
    // zm tn + pzm = zn tn + pzn

    // unknowns: xm, ym, zm, pxm, pym, pzm, t1, t2, t3
    // eq1x: xm t1 + pxm = x1 t1 + px1
    // eq1y: ym t1 + pym = y1 t1 + py1
    // eq1z: zm t1 + pzm = z1 t1 + pz1
    // eq2x: xm t2 + pxm = x2 t2 + px2
    // eq2y: ym t2 + pym = y2 t2 + py2
    // eq2z: zm t2 + pzm = z2 t2 + pz2
    // eq3x: xm t3 + pxm = x3 t3 + px3
    // eq3y: ym t3 + pym = y3 t3 + py3
    // eq3z: zm t3 + pzm = z3 t3 + pz3

    // eq1x -> xm t1 - x1 t1 = px1 - pxm -> t1 (xm - x1) = px1 - pxm
    // ((xm - x1) != 0) -> t1 = (px1 - pxm) / (xm - x1)
    // eq1y -> ym (px1 - pxm) + pym (xm - x1) = y1 (px1 - pxm) + py1 (xm - x1) -> ym (px1 - pxm) + pym (xm - x1) + y1 pxm - xm py1 = y1 px1 - x1 py1
    // eq1z -> zm (px1 - pxm) + pzm (xm - x1) = z1 (px1 - pxm) + pz1 (xm - x1) -> zm (px1 - pxm) + pzm (xm - x1) + z1 pxm - xm pz1 = z1 px1 - x1 pz1

    // eq1y: ym (px1 - pxm) + pym (xm - x1) + y1 pxm - xm py1 = y1 px1 - x1 py1
    // ->    xm pym - ym pxm - xm py1 + ym px1 + y1 pxm - x1 pym = y1 px1 - x1 py1
    // eq1z: xm pzm - zm pxm - xm pz1 + zm px1 + z1 pxm - x1 pzm = z1 px1 - x1 pz1
    // eq2y: xm pym - ym pxm - xm py2 + ym px2 + y2 pxm - x2 pym = y2 px2 - x2 py2

    // unowns: xm, ym, zm, pxm, pym, pzm
    // eq1y - eq2y = eq'1y: xm (py2 - py1) + ym (px1 - px2) + pxm (y1 - y2) + pym (x2 - x1) = y1 px1 - x1 py1 - y2 px2 + x2 py2
    // eq3y - eq4y = eq'2y: xm (py4 - py3) + ym (px3 - px4) + pxm (y3 - y4) + pym (x4 - x3) = y3 px3 - x3 py3 - y4 px4 + x4 py4
    // eq5y - eq6y = eq'3y: xm (py6 - py5) + ym (px5 - px6) + pxm (y5 - y6) + pym (x6 - x5) = y5 px5 - x5 py5 - y6 px6 + x6 py6
    // eq7y - eq8y = eq'4y: xm (py8 - py7) + ym (px7 - px8) + pxm (y7 - y8) + pym (x8 - x7) = y7 px7 - x7 py7 - y8 px8 + x8 py8
    // eq1z - eq2z = eq'1z: xm (pz2 - pz1) + zm (px1 - px2) + pxm (z1 - z2) + pzm (x2 - x1) = z1 px1 - x1 pz1 - z2 px2 + x2 pz2
    // eq3z - eq4z = eq'2z: xm (pz4 - pz3) + zm (px3 - px4) + pxm (z3 - z4) + pzm (x4 - x3) = z3 px3 - x3 pz3 - z4 px4 + x4 pz4

    let hails = parse_input(input);
    if hails.len() < 8 {
        todo!("Not implemented for fewer hails");
    }

    let hail = hails.get(0).unwrap();
    let x1 = hail.trajectory.coords[0];
    let y1 = hail.trajectory.coords[1];
    let z1 = hail.trajectory.coords[2];
    let px1 = hail.position.coords[0];
    let py1 = hail.position.coords[1];
    let pz1 = hail.position.coords[2];

    let hail: &Hail = hails.get(10).unwrap();
    let x2 = hail.trajectory.coords[0];
    let y2 = hail.trajectory.coords[1];
    let z2 = hail.trajectory.coords[2];
    let px2 = hail.position.coords[0];
    let py2 = hail.position.coords[1];
    let pz2 = hail.position.coords[2];

    let hail: &Hail = hails.get(20).unwrap();
    let x3 = hail.trajectory.coords[0];
    let y3 = hail.trajectory.coords[1];
    let z3 = hail.trajectory.coords[2];
    let px3 = hail.position.coords[0];
    let py3 = hail.position.coords[1];
    let pz3 = hail.position.coords[2];

    let hail: &Hail = hails.get(30).unwrap();
    let x4 = hail.trajectory.coords[0];
    let y4 = hail.trajectory.coords[1];
    let z4 = hail.trajectory.coords[2];
    let px4 = hail.position.coords[0];
    let py4 = hail.position.coords[1];
    let pz4 = hail.position.coords[2];

    let hail: &Hail = hails.get(40).unwrap();
    let x5 = hail.trajectory.coords[0];
    let y5 = hail.trajectory.coords[1];
    // let z5 = hail.trajectory.coords[2];
    let px5 = hail.position.coords[0];
    let py5 = hail.position.coords[1];
    // let pz5 = hail.position.coords[2];

    let hail: &Hail = hails.get(50).unwrap();
    let x6 = hail.trajectory.coords[0];
    let y6 = hail.trajectory.coords[1];
    // let z6 = hail.trajectory.coords[2];
    let px6 = hail.position.coords[0];
    let py6 = hail.position.coords[1];
    // let pz6 = hail.position.coords[2];

    let hail: &Hail = hails.get(60).unwrap();
    let x7 = hail.trajectory.coords[0];
    let y7 = hail.trajectory.coords[1];
    // let z7 = hail.trajectory.coords[2];
    let px7 = hail.position.coords[0];
    let py7 = hail.position.coords[1];
    // let pz7 = hail.position.coords[2];

    let hail: &Hail = hails.get(70).unwrap();
    let x8 = hail.trajectory.coords[0];
    let y8 = hail.trajectory.coords[1];
    // let z8 = hail.trajectory.coords[2];
    let px8 = hail.position.coords[0];
    let py8 = hail.position.coords[1];
    // let pz8 = hail.position.coords[2];

    // unowns: xm, ym, zm, pxm, pym, pzm
    // eq1y - eq2y = eq'1y: xm (py2 - py1) + ym (px1 - px2) + pxm (y1 - y2) + pym (x2 - x1) = y1 px1 - x1 py1 - y2 px2 + x2 py2
    // eq3y - eq4y = eq'2y: xm (py4 - py3) + ym (px3 - px4) + pxm (y3 - y4) + pym (x4 - x3) = y3 px3 - x3 py3 - y4 px4 + x4 py4
    // eq5y - eq6y = eq'3y: xm (py6 - py5) + ym (px5 - px6) + pxm (y5 - y6) + pym (x6 - x5) = y5 px5 - x5 py5 - y6 px6 + x6 py6
    // eq7y - eq8y = eq'4y: xm (py8 - py7) + ym (px7 - px8) + pxm (y7 - y8) + pym (x8 - x7) = y7 px7 - x7 py7 - y8 px8 + x8 py8
    // eq1z - eq2z = eq'1z: xm (pz2 - pz1) + zm (px1 - px2) + pxm (z1 - z2) + pzm (x2 - x1) = z1 px1 - x1 pz1 - z2 px2 + x2 pz2
    // eq3z - eq4z = eq'2z: xm (pz4 - pz3) + zm (px3 - px4) + pxm (z3 - z4) + pzm (x4 - x3) = z3 px3 - x3 pz3 - z4 px4 + x4 pz4

    let mut eq = [
        [
            py2 - py1,
            px1 - px2,
            0.0,
            y1 - y2,
            x2 - x1,
            0.0,
            y1 * px1 - x1 * py1 - y2 * px2 + x2 * py2,
        ],
        [
            py4 - py3,
            px3 - px4,
            0.0,
            y3 - y4,
            x4 - x3,
            0.0,
            y3 * px3 - x3 * py3 - y4 * px4 + x4 * py4,
        ],
        [
            pz2 - pz1,
            0.0,
            px1 - px2,
            z1 - z2,
            0.0,
            x2 - x1,
            z1 * px1 - x1 * pz1 - z2 * px2 + x2 * pz2,
        ],
        [
            pz4 - pz3,
            0.0,
            px3 - px4,
            z3 - z4,
            0.0,
            x4 - x3,
            z3 * px3 - x3 * pz3 - z4 * px4 + x4 * pz4,
        ],
        [
            py6 - py5,
            px5 - px6,
            0.0,
            y5 - y6,
            x6 - x5,
            0.0,
            y5 * px5 - x5 * py5 - y6 * px6 + x6 * py6,
        ],
        [
            py8 - py7,
            px7 - px8,
            0.0,
            y7 - y8,
            x8 - x7,
            0.0,
            y7 * px7 - x7 * py7 - y8 * px8 + x8 * py8,
        ],
    ];

    for index_var in 0..6 {
        let q = eq[index_var][index_var];
        assert_ne!(q, 0.0);
        eq[index_var].iter_mut().for_each(|n| *n = *n / q);
        for index_eq in index_var + 1..6 {
            let q = eq[index_eq][index_var];
            eq[index_eq] = eq[index_eq]
                .iter()
                .zip(eq[index_var].iter())
                .map(|(n, b)| n - b * q)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
        }
    }

    for index_var in (0..6).rev() {
        for index_eq in 0..index_var {
            let q = eq[index_eq][index_var];
            eq[index_eq] = eq[index_eq]
                .iter()
                .zip(eq[index_var].iter())
                .map(|(n, b)| n - b * q)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
        }
    }

    // eq.iter().for_each(|v| println!("{:?}", v));

    // let xm = eq[0][6];
    // let ym = eq[1][6];
    // let zm = eq[2][6];
    let pxm = eq[3][6];
    let pym = eq[4][6];
    let pzm = eq[5][6];

    return pxm.round() as usize + pym.round() as usize + pzm.round() as usize;
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
        // eq2 -> y1 (x2 t2 + px2 - px1) / x1 + py1 = y2 t2 + py2
        // -> y1 (x2 t2 + px2 - px1) / x1 = y2 t2 + py2 - py1
        // -> y1 (x2 t2 + px2 - px1) = y2 t2 x1 + py2 x1 - py1 x1
        // -> y1 x2 t2 + y1 px2 - y1 px1 = y2 t2 x1 + py2 x1 - py1 x1
        // -> y1 x2 t2 - y2 t2 x1 = py2 x1 - py1 x1 - y1 px2 + y1 px1
        // -> t2 (y1 x2 - y2 x1) = py2 x1 - py1 x1 - y1 px2 + y1 px1
        // ((y1 x2 - y2 x1) != 0) -> t2 = (py2 x1 - py1 x1 - y1 px2 + y1 px1) / (y1 x2 - y2 x1)
        // ((y1 x2 - y2 x1) = 0) -> py2 x1 - py1 x1 - y1 px2 + y1 px1 = 0

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

        let future_1 = if x1 > &0.0 { &x > px1 } else { px1 > &x };
        let future_2 = if x2 > &0.0 { &x > px2 } else { px2 > &x };
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
        assert!(trajectory
            .coords
            .iter()
            .all(|coord| !compare_floats_equal_enough(coord, &0.0)));
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

    // #[test]
    // fn part2_example() {
    //     let input = get_example_input();
    //     let result = part2(&input);
    //     assert_eq!(result, 47);
    // }
}
