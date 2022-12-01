use std::ops::Range;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = calculate_highest_trajectory(input);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = calculate_trajectory_count(input);
    return result;
}

#[derive(Debug)]
struct TargetArea {
    start_x: i64,
    end_x: i64,
    start_y: i64,
    end_y: i64,
}

impl TargetArea {
    pub fn get_x(&self) -> (i64, i64) {
        (self.start_x, self.end_x)
    }

    pub fn get_start_y(&self) -> i64 {
        self.start_y
    }
    
    pub fn is_within(&self, position: &(i64, i64)) -> bool {
        return position.0 >= self.start_x
            && position.0 <= self.end_x
            && position.1 >= self.start_y
            && position.1 <= self.end_y;
    }
    
    pub fn is_below(&self, position: &(i64, i64)) -> bool {
        return position.1 < self.start_y;
    }

    pub fn is_behind(&self, position: &(i64, i64)) -> bool {
        return position.0 > self.end_x;
    }

    pub fn from(input: &str) -> TargetArea {
        use regex::Regex;

        lazy_static::lazy_static! {
            static ref REGEX: Regex = Regex::new("^target area: x=(-?\\d+)..(-?\\d+), y=(-?\\d+)..(-?\\d+)$").unwrap();
        }
        let captures = REGEX.captures(input).unwrap();
        let start_x = captures[1].parse().unwrap();
        let end_x = captures[2].parse().unwrap();
        let start_y = captures[3].parse().unwrap();
        let end_y = captures[4].parse().unwrap();

        return TargetArea {
            start_x: start_x,
            end_x: end_x,
            start_y: start_y,
            end_y: end_y,
        };
    }
}

fn calculate_highest_trajectory(input: &str) -> i64 {
    let target_area = TargetArea::from(input);
    let y = target_area.get_start_y();
    let height = cumulative_sum(-y - 1);
    return height;
}

fn cumulative_sum(speed: i64) -> i64 {
    if (speed % 2) == 0 {
        let q = speed / 2;
        return speed * q + q;
    } else {
        let q = (speed + 1) / 2;
        return speed * q;
    }
}

fn calculate_trajectory_count(input: &str) -> i64 {
    let target_area = TargetArea::from(input);
    let y = target_area.get_start_y();
    let y_range = y .. -y;
    let x_range = get_valid_x_range(&target_area);

    let mut sum = 0;
    for x in x_range.clone() {
        for y in y_range.clone() {
            if test_trajectory_hits((x, y), &target_area) {
                sum += 1;
            }
        }
    }
    
    return sum;
}

fn test_trajectory_hits(mut trajectory: (i64, i64), target_area: &TargetArea) -> bool {
    let mut position = (0, 0);
    loop {
        position.0 += trajectory.0;
        position.1 += trajectory.1;

        if target_area.is_within(&position) {
            return true;
        }

        if target_area.is_below(&position) || target_area.is_behind(&position) {
            return false;
        }

        if trajectory.0 > 0 {
            trajectory.0 -= 1;
        } else if trajectory.0 < 0 {
            trajectory.0 += 1;
        }
        trajectory.1 -= 1;
    }
}

fn get_valid_x_range(target_area: &TargetArea) -> Range<i64> {

    let area_x = target_area.get_x();
    let mut test_x = 1;
    loop {
        let result = cumulative_sum(test_x);
        
        if result >= area_x.0 && result <= area_x.1 {
            return test_x..area_x.1 + 1;
        }
        test_x += 1;
    }
}

fn get_input() -> &'static str {
    return "target area: x=144..178, y=-100..-76";
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "target area: x=20..30, y=-10..-5";
    }

    macro_rules! cumulative_sum_test {
        ($name: ident, $input: expr, $expected: expr) => {
            #[test]
            fn $name() {
                let result = cumulative_sum($input);
                assert_eq!($expected, result);
            }
        };
    }

    cumulative_sum_test!(cumulative_sum_test1, 1, 1);
    cumulative_sum_test!(cumulative_sum_test5, 5, 15);
    cumulative_sum_test!(cumulative_sum_test6, 6, 21);
    cumulative_sum_test!(cumulative_sum_test10, 10, 55);
    cumulative_sum_test!(cumulative_sum_test11, 11, 66);

    #[test]
    fn example_part1_correct_result() {
        let result = calculate_highest_trajectory(get_example_input());

        assert_eq!(45, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(4950, result);
    }

    macro_rules! get_valid_x_range_test {
        ($name: ident, $start_x: expr, $end_x: expr, $expected: expr) => {
            #[test]
            fn $name() {
                let result = get_valid_x_range(&TargetArea {
                    start_x: $start_x,
                    end_x: $end_x,
                    start_y: 0,
                    end_y: 0,
                });
                assert_eq!($expected, result);
            }
        };
    }
    
    get_valid_x_range_test!(get_valid_x_range_test1, 10, 15, 4..16);
    get_valid_x_range_test!(get_valid_x_range_test2, 20, 30, 6..31);
    get_valid_x_range_test!(get_valid_x_range_test3, 50, 60, 10..61);

    #[test]
    fn example_part2_correct_result() {
        let result = calculate_trajectory_count(get_example_input());

        assert_eq!(112, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(1477, result);
    }
}
