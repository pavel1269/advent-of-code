
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
mod day24;
mod day25;

fn main() {
    let known_solutions = get_known_solutions();
    let (day, part) = get_user_selected_selection(&known_solutions);
    let solution_fn = known_solutions[day - 1][part - 1];
    println!("Result for day {}, part {}: {}", day, part, solution_fn());
}

fn get_user_selected_selection(known_solutions: &Vec<[fn() -> i64; MAX_PARTS]>) -> (usize, usize) {
    let user_input = read_user_selection(&known_solutions);

    println!();
    println!("Selected day '{}' part '{}'", user_input.0, user_input.1);

    return (user_input.0, user_input.1);
}

const MAX_PARTS: usize = 2;

fn get_known_solutions() -> Vec<[fn() -> i64; MAX_PARTS]> {
    let known_solutions = vec![
        [day01::get_solution_part1, day01::get_solution_part2],
        [day02::get_solution_part1, day02::get_solution_part2],
        [day03::get_solution_part1, day03::get_solution_part2],
        [day04::get_solution_part1, day04::get_solution_part2],
        [day05::get_solution_part1, day05::get_solution_part2],
        [day06::get_solution_part1, day06::get_solution_part2],
        [day07::get_solution_part1, day07::get_solution_part2],
        [day08::get_solution_part1, day08::get_solution_part2],
        [day09::get_solution_part1, day09::get_solution_part2],
        [day10::get_solution_part1, day10::get_solution_part2],
        [day11::get_solution_part1, day11::get_solution_part2],
        [day12::get_solution_part1, day12::get_solution_part2],
        [day13::get_solution_part1, day13::get_solution_part2],
        [day14::get_solution_part1, day14::get_solution_part2],
        [day15::get_solution_part1, day15::get_solution_part2],
        [day16::get_solution_part1, day16::get_solution_part2],
        [day17::get_solution_part1, day17::get_solution_part2],
        [day18::get_solution_part1, day18::get_solution_part2],
        [day19::get_solution_part1, day19::get_solution_part2],
        [day20::get_solution_part1, day20::get_solution_part2],
        [day21::get_solution_part1, day21::get_solution_part2],
        [day22::get_solution_part1, day22::get_solution_part2],
        [day23::get_solution_part1, day23::get_solution_part2],
        [day24::get_solution_part1, day24::get_solution_part2],
        [day25::get_solution_part1, get_not_implemented_solution],
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
