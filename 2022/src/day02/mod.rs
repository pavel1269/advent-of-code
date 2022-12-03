
pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = calculate_score(input);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = calculate_score_part2(input);
    return result.to_string();
}

fn calculate_score_part2(input: &str) -> i32 {
    let regex = regex::Regex::new("^([A-Z]) ([A-Z])$").unwrap();

    let mut score = 0;
    input.lines().for_each(|line| {
        let captures = match regex.captures(line) {
            Some(capture) => capture,
            None => panic!("Could not parse '{}'", line),
        };
        let play_opponent = &captures[1];
        let play_my_response = &captures[2];
        let play_my = my_next_play(play_my_response, play_opponent);

        score += score_for_move(play_my.as_str());
        score += score_play(play_my.as_str(), play_opponent);
    });

    return score;
}

fn my_next_play(play_my_response: &str, play_opponent: &str) -> String {
    // Rock AX, Paper BY, Scissors CZ
    // Lose X, draw Y, win Z
    let result = match play_opponent {
        "A" => match play_my_response {
            "X" => "Z",
            "Y" => "X",
            "Z" => "Y",
            _ => panic!("Unexpected move {}", play_my_response),
        },
        "B" => match play_my_response {
            "X" => "X",
            "Y" => "Y",
            "Z" => "Z",
            _ => panic!("Unexpected move {}", play_my_response),
        },
        "C" => match play_my_response {
            "X" => "Y",
            "Y" => "Z",
            "Z" => "X",
            _ => panic!("Unexpected move {}", play_my_response),
        },
        _ => panic!("Unexpected move {}", play_opponent),
    };

    return String::from(result);
}

fn calculate_score(input: &str) -> i32 {
    let regex = regex::Regex::new("^([A-Z]) ([A-Z])$").unwrap();

    let mut score = 0;
    input.lines().for_each(|line| {
        let captures = match regex.captures(line) {
            Some(capture) => capture,
            None => panic!("Could not parse '{}'", line),
        };
        let play_opponent = &captures[1];
        let play_my = &captures[2];

        score += score_for_move(play_my);
        score += score_play(play_my, play_opponent);
    });

    return score;
}

fn score_play(play_my: &str, play_opponent: &str) -> i32 {
    // Rock AX, Paper BY, Scissors CZ
    match play_opponent {
        "A" => match play_my {
            "X" => 3,
            "Y" => 6,
            "Z" => 0,
            _ => panic!("Unexpected move {}", play_my),
        },
        "B" => match play_my {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("Unexpected move {}", play_my),
        },
        "C" => match play_my {
            "X" => 6,
            "Y" => 0,
            "Z" => 3,
            _ => panic!("Unexpected move {}", play_my),
        },
        _ => panic!("Unexpected move {}", play_opponent),
    }
}

fn score_for_move(play_my: &str) -> i32 {
    match play_my {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Unexpected move {}", play_my),
    }
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "A Y
B X
C Z"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = calculate_score(input);

        assert_eq!(result, 15);
    }
    
    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "10718");
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = calculate_score_part2(input);

        assert_eq!(result, 12);
    }

    #[test]
    fn part2_input() {
        let result = get_solution_part2();

        assert_eq!(result, "14652");
    }
}
