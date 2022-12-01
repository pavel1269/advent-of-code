fn main() {
    let input = get_input();
    let result_part1 = most_calories(input);
    let result_part2 = most_calories_top3(input);

    println!("Day 01, part 1 result: {}", result_part1);
    println!("Day 01, part 2 result: {}", result_part2);
}

fn most_calories_top3(input: &str) -> i32 {
    let mut most_calories = vec![0, 0, 0];
    let mut calories = 0;
    input.lines().for_each(|input_line|
    {
        if input_line.len() == 0 {
            update_most_calories(&mut most_calories, calories);
            calories = 0;
        }
        else {
            calories += input_line.parse::<i32>().unwrap();
        }
    });
    
    update_most_calories(&mut most_calories, calories);
    return most_calories[0] + most_calories[1] + most_calories[2];
}

fn update_most_calories(most_calories: &mut Vec<i32>, calories: i32) {
    if calories > most_calories[0] {
        most_calories[2] = most_calories [1];
        most_calories[1] = most_calories [0];
        most_calories[0] = calories;
    }
    else if calories > most_calories[1] {
        most_calories[2] = most_calories [1];
        most_calories[1] = calories;
    }
    else if calories > most_calories[2] {
        most_calories[2] = calories;
    }
}

fn most_calories(input: &str) -> i32 {
    let mut most_calories = 0;
    let mut calories = 0;
    input.lines().for_each(|input_line|
    {
        if input_line.len() == 0 {
            if calories > most_calories {
                most_calories = calories;
            }
            calories = 0;
        }
        else {
            calories += input_line.parse::<i32>().unwrap();
        }
    });
    
    if calories > most_calories {
        most_calories = calories;
    }

    return most_calories;
}

fn get_input() -> &'static str {
    include_str!("./day01/input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example() -> &'static str {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
    }

    #[test]
    fn part1_example() {
        let input = get_example();
        let result = most_calories(input);

        assert_eq!(result, 24000);
    }

    #[test]
    fn part1_input() {
        let input = get_input();
        let result = most_calories(input);

        assert_eq!(result, 70764);
    }

    #[test]
    fn part2_example() {
        let input = get_example();
        let result = most_calories_top3(input);

        assert_eq!(result, 45000);
    }
    
    #[test]
    fn part2_input() {
        let input = get_input();
        let result = most_calories_top3(input);

        assert_eq!(result, 203905);
    }
}
