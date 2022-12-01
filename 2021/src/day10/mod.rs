pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = identify_corrupted_lines(input);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = fix_incomplete(input);
    return result;
}

fn fix_incomplete(input: &str) -> i64 {
    let mut scores = Vec::new();
    for input_line in input.lines() {
        let mut squares = Vec::new();
        let mut corrupted = false;
        for char in input_line.chars() {
            match char {
                '(' => squares.push(')'),
                '[' => squares.push(']'),
                '{' => squares.push('}'),
                '<' => squares.push('>'),
                ')' | ']' | '}' | '>' => {
                    let expected = squares.pop().unwrap();
                    if expected != char {
                        corrupted = true;
                        break;
                    }
                },
                _ => panic!("Unexpected char '{}'", char),
            }
        }

        if !corrupted {
            let mut score = 0;
            while let Some(char) = squares.pop() {
                score *= 5;
                score += match char {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("Unexpected char '{}'", char),
                };
            }
            scores.push(score);
        }
    }

    scores.sort();
    
    return scores[scores.len() / 2];
}

fn identify_corrupted_lines(input: &str) -> i64 {
    let mut sum = 0;
    for input_line in input.lines() {
        let mut squares = Vec::new();
        for char in input_line.chars() {
            match char {
                '(' => squares.push(')'),
                '[' => squares.push(']'),
                '{' => squares.push('}'),
                '<' => squares.push('>'),
                ')' | ']' | '}' | '>' => {
                    let expected = squares.pop().unwrap();
                    if expected != char {
                        sum += match char {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("Unexpected char '{}'", char),
                        };
                        break;
                    }
                },
                _ => panic!("Unexpected char '{}'", char),
            }
        }
    }

    return sum;
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        return "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    }

    #[test]
    fn example_part1_correct_result() {
        let result = identify_corrupted_lines(get_example_input());

        assert_eq!(26397, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(442131, result);
    }

    #[test]
    fn example_part2_correct_result() {
        let result = fix_incomplete(get_example_input());

        assert_eq!(288957, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(3646451424, result);
    }
}
