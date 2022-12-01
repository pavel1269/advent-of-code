
pub fn get_part1_result() -> i64 {
    let adapters = get_challenge_input();
    let result = get_part1_result_adapters(&adapters);
    return result;
}

pub fn get_part2_result() -> i64 {
    let adapters = get_challenge_input();
    let result = get_adapter_combinations(&adapters);
    return result;
}

fn get_adapter_combinations(adapters: &Vec<i64>) -> i64 {
    let mut lengths = get_adapter_combinations_lengths(adapters);
    lengths.sort();

    let largest = *lengths.last().unwrap();
    let mut variants: Vec<i64> = Vec::new();
    variants.push(0);
    for length in 1..largest + 1 {
        variants.push(get_combination_length_variants(length));
    }

    let mut combinations = 1;
    for length in lengths.iter() {
        combinations *= variants[*length as usize];
    }

    return combinations;
}

fn get_combination_length_variants(length: i64) -> i64 {
    length * (length - 1) / 2 + 1
}

fn get_adapter_combinations_lengths(adapters: &Vec<i64>) -> Vec<i64> {
    let mut sorted_adapters = adapters.clone();
    sorted_adapters.sort();

    // 0 3 4 7 -> (1) -> 1
    // 0 3 4 5 8 -> (2) -> 2
    // 0 3 4 5 6 9 -> (3) -> 4
    // 0 3 4 5 6 7 10 -> (4) -> 7
    // 0 3 4 5 6 7 8 11 -> (5) -> 11
    let mut previous: i64 = 0;
    let mut length = 0;
    let mut groups: Vec<i64> = Vec::new();
    for adapter in sorted_adapters.iter() {
        if adapter - previous == 1 {
            length += 1;
        } else {
            if length > 1 {
                groups.push(length);
            }
            length = 0;
        }
        previous = *adapter;
    }
    if length > 1 {
        groups.push(length);
    }

    return groups;
}

fn get_part1_result_adapters(adapters: &Vec<i64>) -> i64 {
    let result = get_jolt_differences(adapters);
    return result[0] * result[2];
}

fn get_jolt_differences(adapters: &Vec<i64>) -> [i64; 3] {
    let mut sorted_adapters = adapters.clone();
    sorted_adapters.sort();

    let mut previous: i64 = 0;
    let mut result: [i64; 3] = [0; 3];
    for adapter in sorted_adapters.iter() {
        let difference = adapter - previous - 1;
        result[difference as usize] += 1;
        previous = *adapter;
    }
    result[2] += 1;

    return result;
}

fn get_challenge_input() -> Vec<i64> {
    include_str!("./inputs/day10.txt").lines().collect::<Vec<&'static str>>().iter().map(|line| line.parse::<i64>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example() -> Vec<i64> {
        vec![
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4,
        ]
    }

    fn get_example2() -> Vec<i64> {
        vec![
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3,
        ]
    }

    #[test]
    fn example_get_part1_result_adapters() {
        let adapters = get_example();
        let result = get_jolt_differences(&adapters);

        assert_eq!([7, 0, 5], result);
    }

    #[test]
    fn example_get_jolt_differences() {
        let adapters = get_example();
        let result = get_part1_result_adapters(&adapters);

        assert_eq!(35, result);
    }

    #[test]
    fn example2_get_part1_result_adapters() {
        let adapters = get_example2();
        let result = get_jolt_differences(&adapters);

        assert_eq!([22, 0, 10], result);
    }

    #[test]
    fn get_part1_res() {
        let result = get_part1_result();

        assert_eq!(2346, result);
    }

    #[test]
    fn example_get_adapter_combinations() {
        let adapters = get_example();
        let result = get_adapter_combinations(&adapters);

        assert_eq!(8, result);
    }

    #[test]
    fn example2_get_adapter_combinations() {
        let adapters = get_example2();
        let result = get_adapter_combinations(&adapters);

        assert_eq!(19208, result);
    }

    #[test]
    fn get_part2_res() {
        let result = get_part2_result();

        assert_eq!(6044831973376, result);
    }
}
