
pub fn get_solution_day01_part1() -> i32 {
    let entries = get_part1_input();
    let result = get_entries_which_sum(2020, entries);

    return result.0 * result.1;
}

fn get_entries_which_sum(sum_value: i32, entries: Vec<i32>) -> (i32, i32) {
    for (index1, entry1) in entries.iter().enumerate() {
        for (index2, entry2) in entries.iter().enumerate() {
            if index1 == index2 {
                continue;
            }

            if entry1 + entry2 == sum_value {
                return (*entry1, *entry2);
            }
        }
    }

    panic!("Could not find any two values wich sum to '{}'", sum_value);
}

fn get_part1_input() -> Vec<i32> {
    let file_contents = include_str!("./inputs/d01p1.txt").split_whitespace();

    let entries = file_contents.map(|entry| match entry.trim().parse::<i32>() {
        Ok(num) => num,
        Err(err) => {
            let error_message = err.to_string() + "\n";
            panic!("Failed to convert number to integer, detail: {}", error_message);
        },
    }).collect();

    return entries;
}

#[allow(dead_code)] // for unit test
fn get_example_input() -> Vec<i32> {
    let input = vec![
        1721,
        979,
        366,
        299,
        675,
        1456,
    ];
    
    return input;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example_input = get_example_input();
        let entries = get_entries_which_sum(2020, example_input);

        assert_eq!((1721, 299), entries);
    }
}
