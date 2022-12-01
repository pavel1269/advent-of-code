//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

// Number - # Segments (if unique) - Segments
// 0 - 6   - abc_efg
// 1 - 2 * - __c__f_
// 2 - 5   - a_cde_g
// 3 - 5   - a_cd_fg
// 4 - 4 * - _bcd_fg
// 5 - 5   - ab_d_fg
// 6 - 6   - ab_defg
// 7 - 3 * - a_c__fg
// 8 - 7 * - abcdefg
// 9 - 6   - abcd_fg
// 5 Segments -> 2/3/5
// 6 Segments -> 0/6/9

// 1   => 0,1,  3,4,    7,8,9   => 1, known: 1
// 4   =>         4,      8,9   => 4, known: 1,4
// 7   => 0,    3,      7,8,9   => 7, known: 1,4,7
// 8   =>                 8     => 8, known: 1,4,7,8

// 4   =>         4,      8,9   => 9, known: 1,4,7,8,9, requires: 4
// !9  => 0,  2,      6,  8
// 1!9 => 0,              8     => 0, known: 0,1,4,7,8,9, requires: 1,4,9
// 1   => 0,1,  3,4,    7,8,9   => 3, known: 0,1,3,4,7,8,9, requires: 0,1,4,9
// !1  =>             6,  8     => 6, known: 0,1,3,4,6,7,8,9, requires: 1
// !9  => 0,  2,      6,  8     => 2, known: 0,1,2,3,4,6,7,8,9, requires: 0,1,6
// missing: 5

mod digit;
use digit::*;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let result = count_unique_output_digits(input);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let result = sum_output_numbers(input);
    return result;
}

fn sum_output_numbers(input: &str) -> i64 {
    let input = parse_input(input);
    let mut sum = 0;
    for (signals, outputs) in input {
        let mut signals: Vec<Digit> = signals.iter().map(|signal| Digit::from(signal)).collect();
        let mut outputs: Vec<Digit> = outputs.iter().map(|signal| Digit::from(signal)).collect();
        let mut all = merge_mut_vec(&mut signals, &mut outputs);

        let one = get_digit_signal(&all, 1);
        let four = get_digit_signal(&all, 4);
        let nine = identify_digits(&mut all, 9, |signal| signal & four == four);
        let nine_neg = !nine & MASK_CLEAN;
        identify_digits(&mut all, 0, |signal| {
            signal & nine_neg == nine_neg && signal & one == one
        });
        identify_digits(&mut all, 3, |signal| signal & one == one);
        let one_neg = !one & MASK_CLEAN;
        identify_digits(&mut all, 6, |signal| signal & one_neg == one_neg);
        identify_digits(&mut all, 2, |signal| signal & nine_neg == nine_neg);
        identify_digits(&mut all, 5, |_| true);

        let mut result: i64 = 0;
        for digit in outputs {
            result *= 10;
            result += digit.get_digit() as i64;
        }
        sum += result;
    }

    return sum;
}

fn identify_digits<F: Fn(u8) -> bool>(
    digits: &mut Vec<&mut Digit>,
    digit: u8,
    digit_test: F,
) -> u8 {
    let identified_digits = digits
        .iter_mut()
        .filter(|digit| !digit.is_any_digit() && digit_test(digit.get_signals()))
        .collect::<Vec<&mut &mut Digit>>();
    assert!(identified_digits.len() > 0);

    let digit_signal = identified_digits[0].get_signals();
    for identified_digit in identified_digits {
        identified_digit.mark_as_digit(digit);
    }

    return digit_signal;
}

fn get_digit_signal(digits: &Vec<&mut Digit>, digit: u8) -> u8 {
    let digit = digits
        .iter()
        .filter(|d| d.is_digit(digit))
        .collect::<Vec<&&mut Digit>>();
    assert!(digit.len() > 0);
    return digit[0].get_signals();
}

fn merge_mut_vec<'a, T>(vec1: &'a mut Vec<T>, vec2: &'a mut Vec<T>) -> Vec<&'a mut T> {
    let mut all: Vec<&mut T> = vec1.iter_mut().collect();
    all.append(&mut vec2.iter_mut().collect::<Vec<&mut T>>());
    return all;
}

fn count_unique_output_digits(input: &str) -> i64 {
    let input = parse_input(input);

    let unique_output_signals = input
        .iter()
        .map(|(_, outputs)| {
            outputs
                .iter()
                .map(|signals| match signals.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                })
                .reduce(|a, b| a + b)
                .unwrap()
        })
        .reduce(|a, b| a + b)
        .unwrap();

    return unique_output_signals as i64;
}

fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    use regex::Regex;
    lazy_static::lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) \| ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7}) ([a-g]{2,7})$").unwrap();
    }

    let result = input
        .lines()
        .map(|input_line| {
            let captures = REGEX.captures(input_line).unwrap();
            let patterns = vec![
                captures[1].to_string(),
                captures[2].to_string(),
                captures[3].to_string(),
                captures[4].to_string(),
                captures[5].to_string(),
                captures[6].to_string(),
                captures[7].to_string(),
                captures[8].to_string(),
                captures[9].to_string(),
                captures[10].to_string(),
            ];
            let outputs = vec![
                captures[11].to_string(),
                captures[12].to_string(),
                captures[13].to_string(),
                captures[14].to_string(),
            ];
            return (patterns, outputs);
        })
        .collect();

    return result;
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input_single() -> &'static str {
        return "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    }

    fn get_example_input() -> &'static str {
        return "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    }

    #[test]
    fn example_part1_correct_result() {
        let result = count_unique_output_digits(get_example_input());

        assert_eq!(26, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(342, result);
    }

    #[test]
    fn example_single_part2_correct_result() {
        let result = sum_output_numbers(get_example_input_single());

        assert_eq!(5353, result);
    }

    #[test]
    fn example_part2_correct_result() {
        let result = sum_output_numbers(get_example_input());

        assert_eq!(61229, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(1068933, result);
    }
}
