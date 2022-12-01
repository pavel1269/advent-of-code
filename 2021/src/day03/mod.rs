
pub fn get_solution_part1() -> i64 {
    let diagnostic = get_input();
    let result = calculate_power_consumption(diagnostic);
    return result;
}

pub fn get_solution_part2() -> i64 {
    let diagnostic = get_input();
    let result = calculate_life_support_rating(diagnostic);
    return result;
}

fn calculate_life_support_rating(diagnostic: Vec<&'static str>) -> i64 {

    let mut filtered_oxygen = diagnostic.clone();
    let mut filtered_co2 = diagnostic.clone();
    for index in 0..diagnostic.len() {
        if filtered_oxygen.len() > 1 {
            let common_bits = process_diagnostic(&filtered_oxygen);
            let (zeroes, ones) = common_bits[index];
            let oxygen_filter_value = if ones >= zeroes {
                '1'
            } else {
                '0'
            };

            filtered_oxygen = filtered_oxygen.iter().filter(|number| number.as_bytes()[index] as char == oxygen_filter_value).cloned().collect();
            // println!("oxygen[{}]: {:?}, filter: {}", index, filtered_oxygen, oxygen_filter_value);
        }
        
        if filtered_co2.len() > 1 {
            let common_bits = process_diagnostic(&filtered_co2);
            let (zeroes, ones) = common_bits[index];
            let co2_filter_value = if zeroes <= ones {
                '0'
            } else {
                '1'
            };

            filtered_co2 = filtered_co2.iter().filter(|number| number.as_bytes()[index] as char == co2_filter_value).cloned().collect();
            // println!("co2[{}]: {:?}, filter: {}", index, filtered_co2, co2_filter_value);
        }
    }

    assert_eq!(filtered_co2.len(), 1);
    assert_eq!(filtered_oxygen.len(), 1);

    let oxygen_rating = i64::from_str_radix(filtered_oxygen[0], 2).unwrap();
    let co2_rating = i64::from_str_radix(filtered_co2[0], 2).unwrap();

    // println!("filtered_oxygen: {}, filtered_co2: {}", filtered_oxygen[0], filtered_co2[0]);

    return oxygen_rating * co2_rating;
}

fn calculate_power_consumption(diagnostic: Vec<&'static str>) -> i64 {
    let common_bits = process_diagnostic(&diagnostic);
    let mut epsilon = 0;
    let mut gamma = 0;
    for (zeroes, ones) in common_bits {
        epsilon <<= 1;
        gamma <<= 1;
        if ones > zeroes {
            epsilon += 1;
        } else {
            gamma += 1;
        }
    }

    return gamma * epsilon;
}

fn process_diagnostic(diagnostic: &Vec<&'static str>) -> Vec<(i32, i32)> {
    let mut common_bits = vec![(0, 0); diagnostic[0].len()];

    for diag_line in diagnostic {
        for (index, bit) in diag_line.chars().enumerate() {
            let is_one = match bit {
                '1' => true,
                _ => false,
            };

            if is_one {
                common_bits[index].1 += 1;
            }
            else {
                common_bits[index].0 += 1;
            }
        }
    }

    return common_bits;
}

fn get_input() -> Vec<&'static str> {
    let file_contents = include_str!("./input.txt").lines().collect();
    return file_contents;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> Vec<&'static str> {
        return vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010",
        ];
    }

    #[test]
    fn example_part1_correct_result() {
        let result = calculate_power_consumption(get_example_input());

        assert_eq!(198, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(3320834, result);
    }

    #[test]
    fn example_part2_correct_result() {
        let result = calculate_life_support_rating(get_example_input());

        assert_eq!(230, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(4481199, result);
    }
}
