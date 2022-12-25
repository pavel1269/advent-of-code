use std::iter::Sum;

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = sum_numbers(input);
    return result;
}

fn sum_numbers(input: &str) -> String {
    let sum = input
        .lines()
        .map(|snafu_number| SnafuNumber::from(snafu_number))
        .sum::<SnafuNumber>();
    return sum.format_snafu();
}

struct SnafuNumber {
    number: i128,
}

impl Sum for SnafuNumber {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let sum = iter.map(|number| number.number).sum();
        return SnafuNumber::new(sum);
    }
}

impl SnafuNumber {
    const WEIGHT: i128 = 5;

    fn format_snafu(&self) -> String {
        let mut approx_power = 1;
        let mut number = self.number;
        while number > 0 {
            number = number / Self::WEIGHT;
            approx_power += 1;
        }

        let weights = Self::get_weights(approx_power);
        let mut number = self.number;
        let mut result = String::with_capacity(approx_power);
        for weight in weights.iter().cloned().rev() {
            let half_weight = weight / 2;
            if number > weight + half_weight {
                result += "2";
                number -= 2 * weight;
            }
            else if number > half_weight {
                result += "1";
                number -= weight;
            }
            else if number <= -(weight + half_weight) {
                result += "=";
                number += 2 * weight;
            }
            else if number < -half_weight {
                result += "-";
                number += weight;
            }
            else {
                if result.len() > 0 {
                    result += "0";
                }
            }
        }

        /*
            Decimal    SNAFU
            1              1
            2              2
            3             1=
            4             1-
            5             10
            6             11
            7             12
            8             2=
            9             2-
            10            20
            11            21
            12            22
            13           1==
            14           1=-
            15           1=0
            16           1=1
            17           1=2
            18           1-=        -7
            19           1--        -6
            20           1-0        -5
            21           1-1        -4
            22           1-2        -3
            23           10=
            24           10-
            25           100
         */
        return result;
    }

    fn from(string: &str) -> SnafuNumber {
        let len = string.len();
        let weights = Self::get_weights(len);

        let mut number = 0;
        for (index, char) in string.chars().enumerate() {
            let index = len - index - 1;
            let power = match char {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!(),
            };
            number += weights[index] * power;
        }

        return SnafuNumber::new(number);
    }

    fn get_weights(len: usize) -> Vec<i128> {
        const LIMIT: usize = 25;
        let mut weights: [i128; LIMIT] = [0; LIMIT];
        weights[0] = 1;
        let weight = Self::WEIGHT;
        let mut previous_weight = 1;
        assert!(len <= LIMIT);

        for index in 1..len {
            let weight_index = previous_weight * weight;
            weights[index] = weight_index;
            previous_weight = weight_index;
        }

        return weights.iter().cloned().filter(|weight| weight > &0).collect();
    }

    fn new(number: i128) -> SnafuNumber {
        SnafuNumber { number }
    }
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(17, "1=2"; "17")]
    #[test_case(20, "1-0"; "20")]
    fn convert_format_snafu(number: i128, expected_result: &str) {
        let number = SnafuNumber::new(number);
        let result = number.format_snafu();
        assert_eq!(result, expected_result);
    }

    #[test_case("1=", 3; "3")]
    #[test_case("1121-1110-1=0", 314159265; "314159265")]
    fn convert_from_snafu(number: &str, expected_result: i128) {
        let result = SnafuNumber::from(number);
        assert_eq!(result.number, expected_result);
    }

    fn get_example_input() -> &'static str {
        "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = sum_numbers(input);

        assert_eq!(result, "2=-1=0");
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "2-0==21--=0==2201==2");
    }
}
