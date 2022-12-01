
pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let answers = count_groups_answers(&input);

    return answers;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let answers = count_groups_unique_answers(&input);

    return answers;
}

fn count_groups_unique_answers(input: &str) -> i64 {
    let answer_groups = parse_answers_groups(&input);

    let mut sum: i64 = 0;
    for answers in answer_groups.iter() {
        sum += count_group_unique_answers(answers);
    }

    return sum;
}

fn count_group_unique_answers(group_answers: &Vec<&str>) -> i64 {
    use std::collections::HashMap;

    let mut answers: HashMap<char, u32> = HashMap::new();
    for character in group_answers[0].chars() {
        answers.insert(character, 1);
    }
    for answer in group_answers[1..].iter() {
        for character in answer.chars() {
            if answers.contains_key(&character) {
                answers.entry(character).and_modify(|i| *i += 1);
            }
        }
    }

    let result = answers.values()
        .filter(|value| (**value) as usize == group_answers.len())
        .collect::<Vec<&u32>>()
        .len();
    return result as i64;
}

fn count_groups_answers(input: &str) -> i64 {
    let answer_groups = parse_answers_groups(&input);
    let answer_group: Vec<String> = answer_groups.iter().map(|group| group.join("")).collect();

    let mut sum: i64 = 0;
    for answers in answer_group.iter() {
        let result = count_group_answers(&answers);
        sum += result as i64;
    }
    return sum;
}

fn count_group_answers(answers: &String) -> i64 {
    use std::collections::HashMap;

    let mut answers_reg: HashMap<char, bool> = HashMap::new();
    for character in answers.chars() {
        if answers_reg.contains_key(&character) {
            continue;
        }

        answers_reg.insert(character, true);
    }

    return answers_reg.len() as i64;
}

fn parse_answers_groups(answers: &str) -> Vec<Vec<&str>> {
    let mut answer_groups: Vec<Vec<&str>> = vec!();
    let mut actual_answers: Vec<&str> = vec!();

    for input_line in answers.lines() {
        if input_line.len() == 0 {
            answer_groups.push(actual_answers.clone());
            actual_answers.clear();
        } else {
            actual_answers.push(input_line.trim());
        }
    }

    if actual_answers.len() > 0 {
        answer_groups.push(actual_answers.clone());
    }

    return answer_groups;
}

fn get_challenge_input() -> &'static str {
    include_str!("./inputs/day06.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &'static str = "abcx
abcy
abcz";

    const EXAMPLE_INPUT_2: &'static str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn example1_parse_groups_correct_len() {
        let parsed_boardpasses = parse_answers_groups(&EXAMPLE_INPUT_1);

        assert_eq!(1, parsed_boardpasses.len());
    }

    #[test]
    fn example2_parse_groups_correct_len() {
        let parsed_boardpasses = parse_answers_groups(&EXAMPLE_INPUT_2);

        assert_eq!(5, parsed_boardpasses.len());
    }

    #[test]
    fn example1_part1_answer_count() {
        let answers = count_groups_answers(&EXAMPLE_INPUT_1);

        assert_eq!(6, answers);
    }

    #[test]
    fn example2_part1_answer_count() {
        let answers = count_groups_answers(&EXAMPLE_INPUT_2);

        assert_eq!(11, answers);
    }

    #[test]
    fn input_part1_answer_count() {
        let input = get_challenge_input();
        let answers = count_groups_answers(&input);

        assert_eq!(6714, answers);
    }

    #[test]
    fn example2_part2_answer_count() {
        let answers = count_groups_unique_answers(&EXAMPLE_INPUT_2);

        assert_eq!(6, answers);
    }

    #[test]
    fn input_part2_answer_count() {
        let input = get_challenge_input();
        let answers = count_groups_unique_answers(&input);

        assert_eq!(3435, answers);
    }
}
