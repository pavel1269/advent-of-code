mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
#[path = "day14.rs"]
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;

fn main() {
    let known_solutions = get_known_solutions();
    let user_input = read_user_selection(&known_solutions);

    println!();
    println!("Selected day '{}' part '{}'", user_input.0, user_input.1);

    let solution_fn = known_solutions[user_input.0 - 1][user_input.1 - 1];
    let result = solution_fn();
    println!("Result: {}", result);
}

const MAX_PARTS: usize = 2;

fn get_known_solutions() -> Vec<[fn() -> i64; MAX_PARTS]> {
    let known_solutions = vec![
        [day01::get_solution_part1, day01::get_solution_part2],
        [day02::get_solution_part1, day02::get_solution_part2],
        [day03::get_solution_part1, day03::get_solution_part2],
        [day04::get_solution_part1, day04::get_solution_part2],
        [day05::get_part1_result, day05::get_part2_result],
        [day06::get_part1_result, day06::get_part2_result],
        [day07::get_part1_result, day07::get_part2_result],
        [day08::get_part1_result, day08::get_part2_result],
        [day09::get_part1_result, day09::get_part2_result],
        [day10::get_part1_result, day10::get_part2_result],
        [day11::get_part1_result, day11::get_part2_result],
        [day12::get_part1_result, day12::get_part2_result],
        [day13::get_part1_result, day13::get_part2_result],
        [day14::get_part1_result, day14::get_part2_result],
        [day15::get_part1_result, day15::get_part2_result],
        [day16::get_part1_result, day16::get_part2_result],
        [day17::get_part1_result, day17::get_part2_result],
        [day18::get_part1_result, day18::get_part2_result],
        [day19::get_part1_result, day19::get_part2_result],
        [day20::get_part1_result, day20::get_part2_result],
        [day21::get_part1_result, day21::get_part2_result],
        [day22::get_part1_result, day22::get_part2_result],
        [day23::get_part1_result, get_not_implemented_solution],
    ];

    return known_solutions;
}

fn get_not_implemented_solution() -> i64 {
    panic!("Trying to retrieve not implemented solution!")
}

fn read_user_selection(known_solutions: &[[fn() -> i64; MAX_PARTS]]) -> (usize, usize) {
    let max_day = known_solutions.len();
    let default_part = if known_solutions[max_day - 1][1] == get_not_implemented_solution {
        1
    } else {
        2
    };

    println!("Welcome to the Advent of code Solution by pavel1269");

    use std::io::Write;
    loop {
        println!();
        println!("Please enter solution day you are interested in.");
        println!("   - Day must be in range <{}, {}>", 1, max_day);
        println!(
            "   - Leave empty for default day '{}' part '{}'",
            max_day, default_part
        );
        print!("Enter day: ");
        std::io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        if input.trim().is_empty() {
            break (max_day, default_part);
        }

        let selected_day: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Could not read input, {}", error);
                continue;
            }
        };

        if selected_day < 1 || selected_day > max_day {
            println!(
                "Day must be <{}, {}>, entered '{}'",
                1, max_day, selected_day
            );
            continue;
        }

        let selected_part = read_user_selection_for_part(selected_day);

        break (selected_day, selected_part);
    }
}

fn read_user_selection_for_part(selected_day: usize) -> usize {
    use std::io::Write;

    loop {
        println!();
        println!("Selected day '{}', now enter which part.", selected_day);
        println!("   - Part must be in range <{}, {}>", 1, MAX_PARTS);
        print!("Enter part: ");
        std::io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        if input.trim().is_empty() {
            continue;
        }

        let selected_part: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Could not read input, {}", error);
                continue;
            }
        };

        if selected_part < 1 || selected_part > 2 {
            println!(
                "Part must be <{}, {}>, entered '{}'",
                1, MAX_PARTS, selected_part
            );
            continue;
        }

        break selected_part;
    }
}
