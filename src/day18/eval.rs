use super::parser::*;

pub fn evaluate_eq(input: &str) -> i64 {
    let mut eq = parse_eq(input);

    let mut index: usize = 0;
    while eq.len() > 1 {
        if index >= eq.len() {
            index = 0;
        }

        println!("[{}] {:?}", index, &eq);
        let token = eq[index];
        match token {
            Token::Number(operand1) => {
                let operation = eq.get(index + 1).cloned().unwrap();
                if operation == Token::ParenthesisClose {
                    index += 3;
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

                eq.remove(index);
                eq.remove(index);
                eq.remove(index);

                let operand2 = operand2.unwrap();
                match operation {
                    Token::OperationPlus => {
                        eq.insert(index, Token::Number(operand1 + operand2));
                    },
                    Token::OperationMultiply => {
                        eq.insert(index, Token::Number(operand1 * operand2));
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    return eq[0].unwrap() as i64;
}

fn solve_partial_eq() {

}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_eq() -> &'static str {
        "1 + 2 * 3 + 4 * 5 + 6"
    }
    
    fn get_example2_eq() -> &'static str {
        "2 * 3 + (4 * 5)"
    }
    
    #[test]
    fn example_evaluate_eq() {
        let input = get_example_eq();
        let result = evaluate_eq(input);
        assert_eq!(71, result);
    }
    
    #[test]
    fn example2_evaluate_eq() {
        let input = get_example2_eq();
        let result = evaluate_eq(input);
        assert_eq!(0, result);
    }
}
