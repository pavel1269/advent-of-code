
pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = calc_coordinates(input);
    return result.to_string();
}

fn calc_coordinates(input: &str) -> i32 {
    let mix = mix_numbers(input);
    let start_index = mix.iter().position(|number| number.1 == 0).unwrap();
    let number1 = mix[(start_index + 1000) % mix.len()];
    let number2 = mix[(start_index + 2000) % mix.len()];
    let number3 = mix[(start_index + 3000) % mix.len()];
    return number1.1 + number2.1 + number3.1;
}

fn mix_numbers(input: &str) -> Vec<(usize, i32)> {
    let numbers = parse_numbers(input);
    let mut decrypt = numbers.clone();
    for number in numbers.iter().copied() {
        //println!("{:?}", decrypt);
        mix_number(&mut decrypt, &number, numbers.len());
    }
    //println!("{:?}", decrypt);

    return decrypt;
}

fn mix_number(decrypt: &mut Vec<(usize, i32)>, number: &(usize, i32), numbers: usize) {
    if number.1 == 0 {
        return;
    }
    
    let numbers = numbers as i32;
    let index = decrypt.iter().position(|decrypt_number| decrypt_number.0 == number.0).unwrap();
    let mut index_new = index as i32 + number.1;
    while index_new < 0 {
        index_new += numbers - 1;
    }
    while index_new >= numbers {
        index_new -= numbers - 1;
    }
    decrypt.remove(index);
    decrypt.insert(index_new as usize, *number);
}

fn parse_numbers(input: &str) -> Vec<(usize, i32)> {
    input.lines().map(|line| line.parse().unwrap()).enumerate().map(|a| a).collect()
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_numbers_easy() {
        let mut numbers: Vec<(usize, i32)> = vec![(0, 0), (1, 0), (2, 2), (3, 0), (4, 1), (5, 3), (6, 0)];
        let size = numbers.len();

        mix_number(&mut numbers, &(2, 2), size);

        assert_eq!(numbers, vec![(0, 0), (1, 0), (3, 0), (4, 1), (2, 2), (5, 3), (6, 0)]);
    }

    #[test]
    fn mix_numbers_wrap_forward() {
        let mut numbers: Vec<(usize, i32)> = vec![(0, 0), (1, 0), (2, 2), (3, 0), (4, 1), (5, 3), (6, 0)];
        let size = numbers.len();

        mix_number(&mut numbers, &(5, 3), size);

        assert_eq!(numbers, vec![(0, 0), (1, 0), (5, 3), (2, 2), (3, 0), (4, 1), (6, 0)]);
    }

    #[test]
    fn mix_numbers_wrap_backward() {
        let mut numbers: Vec<(usize, i32)> = vec![(0, 0), (1, -3), (2, 2), (3, 0), (4, 1), (5, 3), (6, 0)];
        let size = numbers.len();

        mix_number(&mut numbers, &(1, -3), size);

        assert_eq!(numbers, vec![(0, 0), (2, 2), (3, 0), (4, 1), (1, -3), (5, 3), (6, 0)]);
    }

    fn get_example_input() -> &'static str {
        "1
2
-3
3
-2
0
4"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = calc_coordinates(input);

        assert_eq!(result, 3);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "8372");
    }
}
