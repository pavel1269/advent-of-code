
pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = count_increments(&input);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = count_increment_window(&input);
    return result;
}

fn count_increments(measurements: &[i32]) -> i64 {
    let mut last_number = measurements[0];
    let mut increments = 0;
    for &entry in measurements.iter() {
        if entry > last_number {
            increments += 1;
        }
        last_number = entry;
    }

    return increments;
}

fn count_increment_window(measurements: &[i32]) -> i64 {
    let mut window_sum = [0, 0, 0, 0];
    let mut increments = 0;
    for (index, &entry) in measurements.iter().enumerate() {
        window_sum[1] += entry;
        window_sum[2] += entry;
        window_sum[3] += entry;

        // println!("index: {}, [3]: {}, [2]: {}", index, window_sum[1], window_sum[0]);
        if index > 2 && window_sum[1] > window_sum[0] {
            increments += 1;
        }

        window_sum[0] = window_sum[1];
        window_sum[1] = window_sum[2];
        window_sum[2] = window_sum[3];
        window_sum[3] = 0;
    }

    return increments;
}

fn get_input() -> Vec<i32> {
    let file_contents = include_str!("./input.txt").split_whitespace();
    let entries = file_contents.map(|entry| entry.trim().parse().expect("Failed to convert number to integer")).collect();

    return entries;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &[i32] = &[
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263,
    ];

    #[test]
    fn example_part1_correct_result() {
        let result = count_increments(EXAMPLE_INPUT);

        assert_eq!(7, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(1681, result);
    }

    #[test]
    fn example_part2_correct_result() {
        let result = count_increment_window(EXAMPLE_INPUT);

        assert_eq!(5, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(1704, result);
    }
}
