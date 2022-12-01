mod bingo;

use bingo::*;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = get_part1_result(input);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = get_part2_result(input);
    return result;
}

fn get_part1_result(input: &str) -> i64 {
    let (drawn_numbers, mut bingos) = parse_board(input);
    let (number, bingo_index) = draw_bingo_till_first(&drawn_numbers, &mut bingos);
    let sum = bingos[bingo_index].get_sum_unmarked_numbers();
    return sum * number as i64;
}

fn get_part2_result(input: &str) -> i64 {
    let (drawn_numbers, mut bingos) = parse_board(input);
    let (number, bingo_index) = draw_bingo_till_last(&drawn_numbers, &mut bingos);
    let sum = bingo_index.get_sum_unmarked_numbers();
    println!("Sum: '{}', number: '{}'", sum, number);
    return sum * number as i64;
}

fn draw_bingo_till_first(drawn_numbers: &Vec<i32>, bingos: &mut Vec<Bingo>) -> (i32, usize) {
    for number in drawn_numbers {
        for (bingo_index, bingo) in bingos.iter_mut().enumerate() {
            let is_winner = bingo.mark_number(*number);
            if is_winner {
                return (*number, bingo_index);
            }
        }
    }

    panic!("No bingo won");
}

fn draw_bingo_till_last<'a>(drawn_numbers: &Vec<i32>, bingos: &'a mut Vec<Bingo>) -> (i32, &'a Bingo) {
    let mut last_bingo: Option<usize> = None;
    for number in drawn_numbers {
        for bingo in bingos.iter_mut() {
            bingo.mark_number(*number);
        }

        let remaining_bingos: Vec<(usize, &Bingo)> = bingos.iter().enumerate().filter(|(_, bingo)| !bingo.is_won()).collect();
        if remaining_bingos.len() == 1 {
            last_bingo = Some(remaining_bingos[0].0);
        } else if remaining_bingos.len() == 0 {
            return (*number, &bingos[last_bingo.unwrap()]);
        }
    }
    
    panic!();
}

fn parse_board(input: &str) -> (Vec<i32>, Vec<Bingo>) {
    let mut input_enumerate = input.lines().into_iter();
    let drawn_numbers_str = input_enumerate.next().unwrap();
    let drawn_numbers = parse_drawn_numbers(drawn_numbers_str);

    let mut parsed_numbers = Vec::new();
    let mut bingos = Vec::new();

    for input_line in input_enumerate {
        if input_line.len() > 0 {
            let parsed_line = parse_input_line(input_line);
            parsed_numbers.push(parsed_line);
        } else if !parsed_numbers.is_empty() {
            let bingo = Bingo::from(&parsed_numbers);
            bingos.push(bingo);
            parsed_numbers.clear();
        }
    }

    if !parsed_numbers.is_empty() {
        let bingo = Bingo::from(&parsed_numbers);
        bingos.push(bingo);
    }

    return (drawn_numbers, bingos);
}

fn parse_drawn_numbers(drawn_numbers_str: &str) -> Vec<i32> {
    return drawn_numbers_str
        .split(',')
        .map(|number| number.trim().parse().unwrap())
        .collect();
}

fn parse_input_line(input_line: &str) -> Vec<i32> {
    return input_line
        .split_whitespace()
        .map(|number| number.trim().parse().unwrap())
        .collect();
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
    }

    #[test]
    fn example_part1_parsed_correctly() {
        let (drawn_numbers, bingos) = parse_board(get_example_input());

        assert_eq!(27, drawn_numbers.len());
        assert_eq!(3, bingos.len());
    }

    #[test]
    fn example_part1_correct_result() {
        let result = get_part1_result(get_example_input());

        assert_eq!(4512, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(41668, result);
    }

    #[test]
    fn example_part2_correct_result() {
        let result = get_part2_result(get_example_input());

        assert_eq!(1924, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(10478, result);
    }
}
