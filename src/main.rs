
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

// Define these two as of your liking to speed up "run"
const DEFAULT_DAY: usize = 5;
const DEFAULT_PART: usize = 2;

const MAX_PARTS: usize = 2;

fn main() {
    let known_solutions = get_known_solutions();
    let max_day = known_solutions.len();
    let input = read_user_selection(max_day);

    println!();
    println!("Selected day '{}' part '{}'", input.0 + 1, input.1 + 1);

    let solution_fn = known_solutions[input.0][input.1];
    let result = solution_fn();
    println!("Result: {}", result);
}

fn get_known_solutions() -> [[fn() -> i64; MAX_PARTS]; 5] {
    let known_solutions: [[fn() -> i64; MAX_PARTS]; 5] = [
        [
            day01::get_solution_part1,
            day01::get_solution_part2,
        ],
        [
            day02::get_solution_part1,
            day02::get_solution_part2,
        ],
        [
            day03::get_solution_part1,
            day03::get_solution_part2,
        ],
        [
            day04::get_solution_part1,
            day04::get_solution_part2,
        ],
        [
            day05::get_part1_result,
            day05::get_part2_result,
        ],
    ];

    return known_solutions;
}

#[allow(dead_code)]
fn get_not_implemented_solution() -> i64 {
    panic!("Trying to retrieve not implemented solution!")
}

fn read_user_selection(max_day: usize) -> (usize, usize) {
    println!("Welcome to the Advent of code Solution by pavel1269");

    use std::io::Write;
    loop {
        println!();
        println!("Please enter solution day you are interested in.");
        println!("   - Day must be in range <{}, {}>", 1, max_day);
        println!("   - Leave empty for default day '{}' part '{}'", DEFAULT_DAY, DEFAULT_PART);
        print!("Enter day: ");
        std::io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        if input.trim().is_empty() {
            break (DEFAULT_DAY - 1, DEFAULT_PART - 1);
        }
    
        let selected_day: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Could not read input, {}", error);
                continue;
            },
        };

        if selected_day < 1 || selected_day > max_day {
            println!("Day must be <{}, {}>, entered '{}'", 1, max_day, selected_day);
            continue;
        }

        let selected_part = read_user_selection_for_part(selected_day);

        break (selected_day - 1, selected_part - 1);
    }
}

fn read_user_selection_for_part(selected_day: usize) -> usize {
    use std::io::Write;

    loop {
        println!();
        println!("Selected day '{}', now enter which part.", selected_day);
        println!("   - Part must be in range <{}, {}>", 1, MAX_PARTS);
        println!("   - Leave empty for default part '{}'", DEFAULT_PART);
        print!("Enter part: ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        if input.trim().is_empty() {
            break DEFAULT_PART;
        }
    
        let selected_part: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Could not read input, {}", error);
                continue;
            },
        };

        if selected_part < 1 || selected_part > 2 {
            println!("Part must be <{}, {}>, entered '{}'", 1, MAX_PARTS, selected_part);
            continue;
        }

        break selected_part
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use more_asserts::*;

    #[test]
    fn default_part_big_enough() {
        assert_ge!(DEFAULT_PART, 1);
    }

    #[test]
    fn default_part_small_enough() {
        assert_le!(DEFAULT_PART, MAX_PARTS);
    }
    
    #[test]
    fn default_day_big_enough() {
        assert_ge!(DEFAULT_DAY, 1);
    }
    
    #[test]
    fn default_day_smalls_enough() {
        let known_solutions = get_known_solutions().len();

        assert_le!(DEFAULT_DAY, known_solutions);
    }
}
