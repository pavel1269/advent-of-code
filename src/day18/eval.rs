use super::parser::*;

pub fn evaluate_eq_sum(input: &str, plus_precedence: bool) -> i64 {
    let mut sum: i64 = 0;
    for line in input.lines() {
        sum += evaluate_eq(line.trim(), plus_precedence);
    }
    return sum;
}

fn evaluate_eq(input: &str, plus_precedence: bool) -> i64 {
    let mut eq = parse_eq(input);

    let mut index: usize = 0;
    while eq.len() > 1 {
        if index + 1 >= eq.len() {
            index = 0;
        }

        // println!("[{}] {:?}", index, &eq);
        let token = eq[index];
        match token {
            Token::ParenthesisOpen => {
                let token2 = eq.get(index + 1).cloned().unwrap();
                if token2 == Token::ParenthesisOpen {
                    index += 1;
                    continue;
                }

                let token3 = eq.get(index + 2).cloned().unwrap();
                if token3 == Token::ParenthesisClose {
                    eq.remove(index);
                    eq.remove(index + 1);
                    index = 0;
                    continue;
                } else if token3.is_operation() {
                    index += 1;
                    continue;
                }
                panic!();
            }
            Token::Number(operand1) => {
                let operation = eq.get(index + 1).cloned().unwrap();
                if operation == Token::ParenthesisClose {
                    index = 0;
                    continue;
                }
                if !operation.is_operation() {
                    panic!();
                }

                let operand2 = eq[index + 2];
                if operand2 == Token::ParenthesisOpen {
                    index += 2;
                    continue;
                }

                if plus_precedence && operation == Token::OperationMultiply && index + 3 < eq.len() {
                    let token4 = eq[index + 3];
                    if token4 == Token::OperationPlus {
                        index += 2;
                        continue;
                    }
                }

                eq.remove(index);
                eq.remove(index);
                eq.remove(index);

                let operand2 = operand2.unwrap();
                match operation {
                    Token::OperationPlus => {
                        eq.insert(index, Token::Number(operand1 + operand2));
                    }
                    Token::OperationMultiply => {
                        eq.insert(index, Token::Number(operand1 * operand2));
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    return eq[0].unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_eq() -> &'static str {
        "1 + 2 * 3 + 4 * 5 + 6"
    }

    #[test]
    fn example_evaluate_eq() {
        let input = get_example_eq();
        let result = evaluate_eq(input, false);
        assert_eq!(71, result);
    }

    #[test]
    fn example_evaluate_eq_plus_precedence() {
        let input = get_example_eq();
        let result = evaluate_eq(input, true);
        assert_eq!(231, result);
    }

    fn get_example2_eq() -> &'static str {
        "2 * 3 + (4 * 5)"
    }

    #[test]
    fn example2_evaluate_eq() {
        let input = get_example2_eq();
        let result = evaluate_eq(input, false);
        assert_eq!(26, result);
    }

    #[test]
    fn example2_evaluate_eq_plus_precedence() {
        let input = get_example2_eq();
        let result = evaluate_eq(input, true);
        assert_eq!(46, result);
    }

    fn get_example3_eq() -> &'static str {
        "1 + (2 * 3) + (4 * (5 + 6))"
    }

    #[test]
    fn example3_evaluate_eq() {
        let input = get_example3_eq();
        let result = evaluate_eq(input, false);
        assert_eq!(51, result);
    }

    #[test]
    fn example3_evaluate_eq_plus_precedence() {
        let input = get_example3_eq();
        let result = evaluate_eq(input, true);
        assert_eq!(51, result);
    }

    fn get_example4_eq() -> &'static str {
        "5 + (8 * 3 + 9 + 3 * 4 * 3)"
    }

    #[test]
    fn example4_evaluate_eq() {
        let input = get_example4_eq();
        let result = evaluate_eq(input, false);
        assert_eq!(437, result);
    }

    #[test]
    fn example4_evaluate_eq_plus_precedence() {
        let input = get_example4_eq();
        let result = evaluate_eq(input, true);
        assert_eq!(1445, result);
    }

    fn get_example5_eq() -> &'static str {
        "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
    }

    #[test]
    fn example5_evaluate_eq() {
        let input = get_example5_eq();
        let result = evaluate_eq(input, false);
        assert_eq!(12240, result);
    }

    #[test]
    fn example5_evaluate_eq_plus_precedence() {
        let input = get_example5_eq();
        let result = evaluate_eq(input, true);
        assert_eq!(669060, result);
    }

    fn get_example6_eq() -> &'static str {
        "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
        // normal
        // 1 - 6 * 9 ...
        // 2 - 54 * ...
        // 3 - 54 * (15 * 8 + 6) ...
        // 4 - 54 * (120 + 6) ...
        // 5 - 54 * 126 ...
        // 6 - 6804 + 6 + 2 + 4 * 2
        // 7 - 6810 + 2 + 4 * 2
        // 8 - 6812 + 4 * 2
        // 9 - 6816 * 2
        // 10 - 13632
    }

    #[test]
    fn example6_evaluate_eq() {
        let input = get_example6_eq();
        let result = evaluate_eq(input, false);
        assert_eq!(13632, result);
    }

    #[test]
    fn example6_evaluate_eq_plus_precedence() {
        let input = get_example6_eq();
        let result = evaluate_eq(input, true);
        assert_eq!(23340, result);
    }

    #[test]
    fn examples_evaluate_eq_sum() {
        let input = format!(
            "{}\n{}\n{}\n{}",
            get_example_eq(),
            get_example2_eq(),
            get_example3_eq(),
            get_example_eq(),
        );
        let result = evaluate_eq_sum(input.as_str(), false);
        assert_eq!(71 + 26 + 51 + 71, result);
    }
}
