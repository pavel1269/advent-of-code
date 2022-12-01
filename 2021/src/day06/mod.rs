
pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = calculate_number_of_fish(80, input);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = calculate_number_of_fish(256, input);
    return result;
}

fn calculate_number_of_fish(days: usize, fish_timers: &str) -> i64 {
    const NEW_FISH_TIMER: usize = 8;
    const FISH_RESET_TIMER: usize = 6;
    let fishes = parse_input(fish_timers);
    let mut fishes_counts = vec![0; NEW_FISH_TIMER + 1];
    for counter in fishes {
        fishes_counts[counter] += 1;
    }

    for _ in 0..days {
        let zeroes = fishes_counts[0];
        for counter in 0..NEW_FISH_TIMER {
            fishes_counts[counter] = fishes_counts[counter + 1];
        }
        fishes_counts[FISH_RESET_TIMER] += zeroes;
        fishes_counts[NEW_FISH_TIMER] = zeroes;
    }

    let fishes = fishes_counts.iter().copied().reduce(|a, b| a + b).unwrap();
    return fishes;
}

fn parse_input(input: &str) -> Vec<usize> {
    return input.trim().split(',').map(|clock| clock.parse().unwrap()).collect();
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "3,4,3,1,2";
    }

    #[test]
    fn example_18days_correct_result() {
        let result = calculate_number_of_fish(18, get_example_input());

        assert_eq!(26, result);
    }
    
    #[test]
    fn example_80days_correct_result() {
        let result = calculate_number_of_fish(80, get_example_input());

        assert_eq!(5934, result);
    }
    
    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(387413, result);
    }
    
    #[test]
    fn example_256days_correct_result() {
        let result = calculate_number_of_fish(256, get_example_input());

        assert_eq!(26984457539, result);
    }
    
    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(1738377086345, result);
    }
}
