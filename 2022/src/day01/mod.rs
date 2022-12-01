
pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = most_calories(input);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = most_calories_top3(input);
    return result.to_string();
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
    include_str!("./input.txt")
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
        let result = get_solution_part1();

        assert_eq!(result, "70764");
    }

    #[test]
    fn part2_example() {
        let input = get_example();
        let result = most_calories_top3(input);

        assert_eq!(result, 45000);
    }
    
    #[test]
    fn part2_input() {
        let result = get_solution_part2();

        assert_eq!(result, "203905");
    }
}
