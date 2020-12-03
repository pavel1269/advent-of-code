
pub fn get_solution_part1() -> i64 {
    let entries = get_input();
    let result = get_two_entries_which_sum(2020, &entries).unwrap();

    return (result.0 * result.1) as i64;
}

pub fn get_solution_part2() -> i64 {
    let entries = get_input();
    let result = get_three_entries_which_sum(2020, &entries).unwrap();

    return (result.0 * result.1 * result.2) as i64;
}

fn get_two_entries_which_sum(sum_value: i32, entries: &[i32]) -> Result<(i32, i32), Box<String>> {
    for (index1, entry1) in entries.iter().enumerate() {
        for (index2, entry2) in entries.iter().enumerate() {
            if index1 == index2 {
                continue;
            }

            if entry1 + entry2 == sum_value {
                return Ok((*entry1, *entry2));
            }
        }
    }

    let error = Box::new(format!("Could not find any two values wich sum to '{}'", sum_value));
    return Err(error);
}

fn get_three_entries_which_sum(sum_value: i32, entries: &[i32]) -> Result<(i32, i32, i32), Box<String>> {
    for (index1, entry1) in entries.iter().enumerate() {
        for (index2, entry2) in entries.iter().enumerate() {
            if index1 == index2 {
                continue;
            }

            for (index3, entry3) in entries.iter().enumerate() {
                if index1 == index3 {
                    continue;
                }
                if index2 == index3 {
                    continue;
                }

                if entry1 + entry2 + entry3 == sum_value {
                    return Ok((*entry1, *entry2, *entry3));
                }
            }
        }
    }

    let error = Box::new(format!("Could not find any three values wich sum to '{}'", sum_value));
    return Err(error);
}

fn get_input() -> Vec<i32> {
    let file_contents = include_str!("./inputs/day01.txt").split_whitespace();
    let entries = file_contents.map(|entry| entry.trim().parse().expect("Failed to convert number to integer")).collect();

    return entries;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &[i32] = &[
        1721,
        979,
        366,
        299,
        675,
        1456,
    ];

    #[test]
    fn example_part1_correct_result() {
        let entries = get_two_entries_which_sum(2020, &EXAMPLE_INPUT).unwrap();

        assert_eq!((1721, 299), entries);
    }

    #[test]
    fn input_part1_correct_result() {
        let expected_result = 545379;
        let result = get_solution_part1();

        assert_eq!(expected_result, result);
    }

    #[test]
    fn example_part2_correct_result() {
        let entries = get_three_entries_which_sum(2020, &EXAMPLE_INPUT).unwrap();

        assert_eq!((979, 366, 675), entries);
    }

    #[test]
    fn input_part2_correct_result() {
        let expected_result = 257778836;
        let result = get_solution_part2();

        assert_eq!(expected_result, result);
    }
}
