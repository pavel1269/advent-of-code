use super::*;

impl SnailNumber {
    pub fn parse(input: &str) -> SnailNumber {
        let mut stack = Vec::new();
        for char in input.trim().chars() {
            match char {
                ',' => continue,
                '[' => stack.push(ParseToken::create_open_bracket()),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    stack.push(ParseToken::create_normal_number(char))
                }
                ']' => {
                    let right_number = stack.pop().unwrap();
                    let left_number = stack.pop().unwrap();
                    if !stack.pop().unwrap().is_open_bracket() {
                        panic!();
                    }
                    let number = SnailNumber::create_parse(left_number, right_number);
                    stack.push(ParseToken::create_snail_number(&number));
                }
                _ => panic!(),
            }
        }

        assert_eq!(1, stack.len(), "stack: {:?}", stack);
        return stack.pop().unwrap().get_snail_number();
    }

    fn create_parse(left_number: ParseToken, right_number: ParseToken) -> SnailNumber {
        let is_number1_snail = left_number.is_snail_number();
        let is_number2_snail = right_number.is_snail_number();

        let mut number: SnailNumber = Default::default();

        if is_number1_snail {
            number.left_snail_number = Some(Box::from(left_number.get_snail_number()));
        } else {
            number.left_normal_number = Some(left_number.get_normal_number());
        }

        if is_number2_snail {
            number.right_snail_number = Some(Box::from(right_number.get_snail_number()));
        } else {
            number.right_normal_number = Some(right_number.get_normal_number());
        }

        return number;
    }
}

#[derive(Debug)]
struct ParseToken {
    is_open_bracket: bool,
    normal_number: Option<Number>,
    snail_number: Option<SnailNumber>,
}

impl ParseToken {
    pub fn is_open_bracket(&self) -> bool {
        self.is_open_bracket
    }

    pub fn is_snail_number(&self) -> bool {
        self.snail_number.is_some()
    }

    pub fn get_snail_number(&self) -> SnailNumber {
        self.snail_number.as_ref().unwrap().clone()
    }

    pub fn get_normal_number(&self) -> Number {
        self.normal_number.unwrap()
    }

    pub fn create_snail_number(snail_number: &SnailNumber) -> ParseToken {
        ParseToken {
            normal_number: None,
            is_open_bracket: false,
            snail_number: Some(snail_number.clone()),
        }
    }

    pub fn create_normal_number(normal_number: char) -> ParseToken {
        ParseToken {
            normal_number: Some((normal_number as u8 - '0' as u8) as Number),
            is_open_bracket: false,
            snail_number: None,
        }
    }

    pub fn create_open_bracket() -> ParseToken {
        ParseToken {
            normal_number: None,
            is_open_bracket: true,
            snail_number: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snail_number_parse_test1() {
        let result = SnailNumber::parse("[1,2]");

        assert_eq!(1, result.get_left_normal_number());
        assert_eq!(2, result.get_right_normal_number());
    }

    #[test]
    fn snail_number_parse_test2() {
        let result = SnailNumber::parse("[[1,9],[8,5]]");
        let left = result.get_left_snail_number();
        let right = result.get_right_snail_number();

        assert_eq!(1, left.get_left_normal_number());
        assert_eq!(9, left.get_right_normal_number());
        assert_eq!(8, right.get_left_normal_number());
        assert_eq!(5, right.get_right_normal_number());
    }

    #[test]
    fn snail_number_parse_test3() {
        let result = SnailNumber::parse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");

        // [
        //     [
        //         [
        //             [1,2],
        //             [3,4]
        //         ],[
        //             [5,6],
        //             [7,8]
        //         ]
        //     ],9
        // ]

        assert_eq!(9, result.get_right_normal_number());

        let number = result.get_left_snail_number();
        let left1 = number.get_left_snail_number();
        
        let right = left1.get_right_snail_number();
        let left = left1.get_left_snail_number();
        
        assert_eq!(1, left.get_left_normal_number());
        assert_eq!(2, left.get_right_normal_number());
        assert_eq!(3, right.get_left_normal_number());
        assert_eq!(4, right.get_right_normal_number());
        
        let right1 = number.get_right_snail_number();
        let left = right1.get_left_snail_number();
        let right = right1.get_right_snail_number();
        
        assert_eq!(5, left.get_left_normal_number());
        assert_eq!(6, left.get_right_normal_number());
        assert_eq!(7, right.get_left_normal_number());
        assert_eq!(8, right.get_right_normal_number());
    }
}
