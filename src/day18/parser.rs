#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    OperationPlus,
    OperationMultiply,
    ParenthesisOpen,
    ParenthesisClose,
}

enum ParserState {
    Default,
    Number,
}

pub fn parse_eq(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut state = ParserState::Default;
    let mut buf: i32 = 0;

    let char_array = input.chars().collect::<Vec<char>>();
    let index_max = char_array.len();
    let mut index: usize = 0;
    while index < index_max {
        let char = char_array[index];
        match state {
            ParserState::Number => {
                if char.is_numeric() {
                    buf *= 10;
                    buf += char.to_digit(10).unwrap() as i32;
                    index += 1;
                } else {
                    tokens.push(Token::Number(buf));
                    buf = 0;
                    state = ParserState::Default;
                }
            }
            ParserState::Default => {
                if char.is_numeric() {
                    state = ParserState::Number;
                    buf += char.to_digit(10).unwrap() as i32;
                } else {
                    match char {
                        '(' => tokens.push(Token::ParenthesisOpen),
                        ')' => tokens.push(Token::ParenthesisClose),
                        '+' => tokens.push(Token::OperationPlus),
                        '*' => tokens.push(Token::OperationMultiply),
                        ' ' => {}
                        _ => panic!(format!("Unexpected char '{}'", char)),
                    }
                }
                index += 1;
            }
        }
    }

    if buf != 0 {
        tokens.push(Token::Number(buf));
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::Token::*;
    use super::*;

    fn get_example_eq() -> &'static str {
        "1 + 2 * 3 + 4 * 5 + 6"
    }

    fn get_example2_eq() -> &'static str {
        "(1 + 2) + (4 * (5 + 6))"
    }

    #[test]
    fn example_eq_parsed() {
        let input = get_example_eq();
        let result = parse_eq(input);
        let expected = vec![
            Number(1),
            OperationPlus,
            Number(2),
            OperationMultiply,
            Number(3),
            OperationPlus,
            Number(4),
            OperationMultiply,
            Number(5),
            OperationPlus,
            Number(6),
        ];
        assert_eq!(&expected, &result);
    }

    #[test]
    fn example2_eq_parsed() {
        let input = get_example2_eq();
        let result = parse_eq(input);
        let expected = vec![
            ParenthesisOpen,
            Number(1),
            OperationPlus,
            Number(2),
            ParenthesisClose,
            OperationPlus,
            ParenthesisOpen,
            Number(4),
            OperationMultiply,
            ParenthesisOpen,
            Number(5),
            OperationPlus,
            Number(6),
            ParenthesisClose,
            ParenthesisClose,
        ];
        assert_eq!(&expected, &result);
    }
}
