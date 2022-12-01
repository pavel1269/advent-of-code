mod snail_number_parser;

use std::{fmt::Debug, panic};

type Number = i8;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let number = sum_snail_numbers(input);
    let result = number.get_magnitude();
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = largest_sum(input);
    return result;
}

#[derive(Clone)]
struct SnailNumber {
    left_normal_number: Option<Number>,
    right_normal_number: Option<Number>,
    left_snail_number: Option<Box<SnailNumber>>,
    right_snail_number: Option<Box<SnailNumber>>,
}

impl SnailNumber {
    pub fn is_left_snail(&self) -> bool {
        self.validate();
        self.left_snail_number.is_some()
    }

    pub fn is_right_snail(&self) -> bool {
        self.validate();
        self.right_snail_number.is_some()
    }

    pub fn get_magnitude(&self) -> i64 {
        let left = if self.is_left_snail() {
            self.get_left_snail_number().get_magnitude()
        } else {
            self.get_left_normal_number() as i64
        };

        let right = if self.is_right_snail() {
            self.get_right_snail_number().get_magnitude()
        } else {
            self.get_right_normal_number() as i64
        };

        return left * 3 + right * 2;
    }

    pub fn add(&self, other: &SnailNumber) -> SnailNumber {
        // println!("= {:?}", self);
        // println!("+ {:?}", other);
        let mut add_result = SnailNumber::create(&self, other);
        // println!("after add:      {:?}", add_result);
        add_result.reduce();
        // println!("= {:?}", add_result);
        // println!();
        return add_result;
    }

    fn reduce(&mut self) {
        let mut reduce = true;
        while reduce {
            while let Some(_) = SnailNumber::reduce_explode(self, 0) {
                // println!("after explode:  {:?}", self);
            }
            reduce = self.reduce_split();
            if reduce {
                // println!("after split:    {:?}", self);
            }
        }
    }

    fn reduce_split(&mut self) -> bool {
        let is_left_snail_number = self.is_left_snail();
        let is_right_snail_number = self.is_right_snail();

        if is_left_snail_number {
            let result = self.get_left_snail_number_mut().reduce_split();
            if result {
                return true;
            }
        } else {
            let left = self.get_left_normal_number();
            if left > 9 {
                self.left_normal_number = None;
                self.left_snail_number = Some(Box::from(SnailNumber::produce_split(left)));
                self.validate();
                return true;
            }
        }

        if is_right_snail_number {
            let result = self.get_right_snail_number_mut().reduce_split();
            if result {
                return true;
            }
        } else {
            let right = self.get_right_normal_number();
            if right > 9 {
                self.right_normal_number = None;
                self.right_snail_number = Some(Box::from(SnailNumber::produce_split(right)));
                self.validate();
                return true;
            }
        }

        return false;
    }

    fn produce_split(value: Number) -> SnailNumber {
        let new_left = value / 2;
        let new_right = value - new_left;
        let snail_number = SnailNumber {
            left_normal_number: Some(new_left),
            right_normal_number: Some(new_right),
            ..Default::default()
        };
        return snail_number;
    }

    fn reduce_explode(&mut self, deepness: usize) -> Option<(bool, (Number, Number))> {
        let is_left_snail_number = self.is_left_snail();
        let is_right_snail_number = self.is_right_snail();

        if deepness >= 4 {
            if !is_left_snail_number && !is_right_snail_number {
                return Some((
                    true,
                    (
                        self.get_left_normal_number(),
                        self.get_right_normal_number(),
                    ),
                ));
            }
            todo!();
        }

        if is_left_snail_number {
            let result = self
                .get_left_snail_number_mut()
                .reduce_explode(deepness + 1);
            if let Some((should_reduce, (left, mut right))) = result {
                if should_reduce {
                    self.left_snail_number = None;
                    self.left_normal_number = Some(0);
                }
                if right > 0 {
                    if is_right_snail_number {
                        self.get_right_snail_number_mut().add_from_right(right);
                    } else {
                        self.right_normal_number = Some(self.get_right_normal_number() + right);
                    }
                    right = 0;
                }
                self.validate();
                return Some((false, (left, right)));
            }
        }
        if is_right_snail_number {
            let result = self
                .get_right_snail_number_mut()
                .reduce_explode(deepness + 1);
            if let Some((should_reduce, (mut left, right))) = result {
                if should_reduce {
                    self.right_snail_number = None;
                    self.right_normal_number = Some(0);
                }
                if left > 0 {
                    if is_left_snail_number {
                        self.get_left_snail_number_mut().add_from_left(left);
                    } else {
                        self.left_normal_number = Some(self.get_left_normal_number() + left);
                    }
                    left = 0;
                }
                self.validate();
                return Some((false, (left, right)));
            }
        }
        return None;
    }

    fn add_from_right(&mut self, number: Number) {
        if self.is_left_snail() {
            self.get_left_snail_number_mut().add_from_right(number);
        } else {
            self.left_normal_number = Some(self.get_left_normal_number() + number);
        }
    }

    fn add_from_left(&mut self, number: Number) {
        if self.is_right_snail() {
            self.get_right_snail_number_mut().add_from_left(number);
        } else {
            self.right_normal_number = Some(self.get_right_normal_number() + number);
        }
    }

    fn get_right_normal_number(&self) -> Number {
        self.validate();
        self.right_normal_number.unwrap()
    }

    fn get_left_normal_number(&self) -> Number {
        self.validate();
        self.left_normal_number.unwrap()
    }

    #[allow(dead_code)]
    fn get_right_snail_number(&self) -> SnailNumber {
        self.validate();
        self.right_snail_number.as_ref().unwrap().as_ref().clone()
    }

    #[allow(dead_code)]
    fn get_left_snail_number(&self) -> SnailNumber {
        self.validate();
        self.left_snail_number.as_ref().unwrap().as_ref().clone()
    }

    fn get_right_snail_number_mut(&mut self) -> &mut SnailNumber {
        self.validate();
        self.right_snail_number.as_mut().unwrap().as_mut()
    }

    fn get_left_snail_number_mut(&mut self) -> &mut SnailNumber {
        self.validate();
        self.left_snail_number.as_mut().unwrap().as_mut()
    }

    fn validate(&self) {
        assert_eq!(
            self.left_normal_number.is_none(),
            self.left_snail_number.is_some()
        );
        assert_eq!(
            self.right_normal_number.is_none(),
            self.right_snail_number.is_some()
        );
    }

    fn create(&self, right: &SnailNumber) -> SnailNumber {
        SnailNumber {
            left_snail_number: Some(Box::from(self.clone())),
            right_snail_number: Some(Box::from(right.clone())),
            ..Default::default()
        }
    }
}

impl Default for SnailNumber {
    fn default() -> Self {
        Self {
            left_normal_number: None,
            right_normal_number: None,
            left_snail_number: None,
            right_snail_number: None,
        }
    }
}

impl Debug for SnailNumber {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let argument1 = if self.is_left_snail() {
            format!("{:?}", self.left_snail_number.as_ref().unwrap().as_ref())
        } else {
            format!("{}", self.get_left_normal_number())
        };

        let argument2 = if self.is_right_snail() {
            format!("{:?}", self.right_snail_number.as_ref().unwrap().as_ref())
        } else {
            format!("{}", self.get_right_normal_number())
        };

        return formatter.write_str(&format!("[{},{}]", argument1, argument2));
    }
}

fn largest_sum(input: &str) -> i64 {
    let numbers = input.lines().map(|line| SnailNumber::parse(line)).collect::<Vec<_>>();
    let mut largest_sum = 0;
    for (index, number) in numbers.iter().enumerate() {
        for index2 in index + 1..numbers.len() {
            let number2 = &numbers[index2];

            let magnitude = number.add(number2).get_magnitude();
            if largest_sum < magnitude {
                largest_sum = magnitude;
            }

            let magnitude = number2.add(number).get_magnitude();
            if largest_sum < magnitude {
                largest_sum = magnitude;
            }
        }
    }
    return largest_sum;
}

fn sum_snail_numbers(input: &str) -> SnailNumber {
    let mut number = None;
    for line in input.lines() {
        let other = SnailNumber::parse(line);
        if number.is_none() {
            number = Some(other);
        } else {
            number = Some(number.as_mut().unwrap().add(&other));
        }
    }
    return number.unwrap();
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snail_number_add_test() {
        let a = SnailNumber::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = SnailNumber::parse("[1,1]");
        let result = a.add(&b);

        assert_eq!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", format!("{:?}", result));
    }

    #[test]
    fn sum_snail_numbers_test1() {
        let result = sum_snail_numbers(
            "[1,1]
[2,2]
[3,3]
[4,4]",
        );

        assert_eq!("[[[[1,1],[2,2]],[3,3]],[4,4]]", format!("{:?}", result));
    }

    #[test]
    fn sum_snail_numbers_test2() {
        let result = sum_snail_numbers(
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]",
        );

        assert_eq!("[[[[3,0],[5,3]],[4,4]],[5,5]]", format!("{:?}", result));
    }

    #[test]
    fn sum_snail_numbers_test3() {
        let result = sum_snail_numbers(
            "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]",
        );

        assert_eq!("[[[[5,0],[7,4]],[5,5]],[6,6]]", format!("{:?}", result));
    }

    #[test]
    fn sum_snail_numbers_test4() {
        let result = sum_snail_numbers(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]",
        );

        assert_eq!(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            format!("{:?}", result)
        );
    }

    fn get_example_input() -> &'static str {
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
    }

    #[test]
    fn sum_snail_numbers_test_example() {
        let result = sum_snail_numbers(get_example_input());

        assert_eq!(
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            format!("{:?}", result)
        );
    }

    macro_rules! snail_number_get_magnitude_test {
        ($name: ident, $input: expr, $expected: expr) => {
            #[test]
            fn $name() {
                let resullt = SnailNumber::parse($input).get_magnitude();
                assert_eq!($expected, resullt);
            }
        };
    }

    snail_number_get_magnitude_test!(snail_number_get_magnitude_test1, "[9,1]", 29);
    snail_number_get_magnitude_test!(snail_number_get_magnitude_test2, "[1,9]", 21);
    snail_number_get_magnitude_test!(snail_number_get_magnitude_test3, "[[9,1],[1,9]]", 129);
    snail_number_get_magnitude_test!(snail_number_get_magnitude_test4, "[[1,2],[[3,4],5]]", 143);
    snail_number_get_magnitude_test!(
        snail_number_get_magnitude_test5,
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        1384
    );
    snail_number_get_magnitude_test!(
        snail_number_get_magnitude_test6,
        "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        445
    );
    snail_number_get_magnitude_test!(
        snail_number_get_magnitude_test7,
        "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        791
    );
    snail_number_get_magnitude_test!(
        snail_number_get_magnitude_test8,
        "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        1137
    );
    snail_number_get_magnitude_test!(
        snail_number_get_magnitude_test9,
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        3488
    );

    #[test]
    fn part1_example() {
        let result = sum_snail_numbers(get_example_input());
        assert_eq!(4140, result.get_magnitude());
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();
        assert_eq!(4116, result);
    }

    #[test]
    fn part2_example() {
        let result = largest_sum(get_example_input());
        assert_eq!(3993, result);
    }

    #[test]
    fn part2_input() {
        let result = get_solution_part2();
        assert_eq!(4638, result);
    }
}
