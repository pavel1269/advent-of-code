
pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = get_fuel_required_to_align_linear(input);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = get_fuel_required_to_align_increasing(input);
    return result;
}

fn get_fuel_required_to_align_linear(input: &str) -> i64 {
    let mut positions = parse_input(input);
    positions.sort();
    let middle = positions[positions.len() / 2] as i64;
    let result: i64 = positions.iter().copied().map(|position| (position as i64 - middle).abs()).sum();
    return result;
}

fn get_fuel_required_to_align_increasing(input: &str) -> i64 {
    let mut positions_int = parse_input(input);
    positions_int.sort();
    let positions_float = positions_int.iter().copied().map(|position| position as f64).collect::<Vec<f64>>();
    let sum: f64 = positions_float.iter().copied().sum();
    let average_min = (sum / positions_float.len() as f64).floor() as i64;
    let average_max = (sum / positions_float.len() as f64).ceil() as i64;

    let differences_min: Vec<i64> = positions_int.iter().copied().map(|position| (position as i64 - average_min).abs()).collect();
    let differences_max: Vec<i64> = positions_int.iter().copied().map(|position| (position as i64 - average_max).abs()).collect();

    let max_diff_min = differences_min.iter().copied().max().unwrap();
    let max_diff_max = differences_max.iter().copied().max().unwrap();
    let max_diff = max_diff_min.max(max_diff_max);

    let cache = build_fuel_cache(max_diff);

    let result_min: i64 = differences_min.iter().copied().map(|difference| cache[difference as usize]).sum();
    let result_max = differences_max.iter().copied().map(|difference| cache[difference as usize]).sum();

    return result_min.min(result_max);
}

fn build_fuel_cache(max_diff: i64) -> Vec<i64> {
    let mut sum = 0;
    let mut cache = vec![0; max_diff as usize + 1];
    for index in 0..max_diff+1 {
        sum += index;
        cache[index as usize] = sum;
    }

    return cache;
}

fn parse_input(input: &str) -> Vec<i32> {
    return input.trim().split(',').map(|clock| clock.parse().unwrap()).collect();
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "16,1,2,0,4,2,7,1,2,14";
    }

    #[test]
    fn example_part1_correct_result() {
        let result = get_fuel_required_to_align_linear(get_example_input());

        assert_eq!(37, result);
    }
    
    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(351901, result);
    }

    #[test]
    fn example_part2_correct_result() {
        let result = get_fuel_required_to_align_increasing(get_example_input());

        assert_eq!(168, result);
    }
    
    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(101079875, result);
    }
}
