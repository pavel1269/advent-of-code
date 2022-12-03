
mod day01;
mod day02;
mod day03;

fn main() {
    let known_solutions = get_known_solutions();
    let (day, part) = get_user_selected_selection(&known_solutions);
    let solution_fn = known_solutions[day - 1][part - 1];
    println!("Result for day {}, part {}: {}", day, part, solution_fn());
}

const MAX_PARTS: usize = 2;

fn get_known_solutions() -> Vec<[fn() -> String; MAX_PARTS]> {
    let known_solutions = vec![
        [day01::get_solution_part1, day01::get_solution_part2],
        [day02::get_solution_part1, day02::get_solution_part2],
        [day03::get_solution_part1, day03::get_solution_part2],
    ];

    return known_solutions;
}

fn get_not_implemented_solution() -> String {
    panic!("Trying to retrieve not implemented solution!")
}

fn get_user_selected_selection(known_solutions: &Vec<[fn() -> String; MAX_PARTS]>) -> (usize, usize) {
    let user_input = read_user_selection(&known_solutions);

    println!();
    println!("Selected day '{}' part '{}'", user_input.0, user_input.1);

    return (user_input.0, user_input.1);
}

fn read_user_selection(known_solutions: &[[fn() -> String; MAX_PARTS]]) -> (usize, usize) {
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
