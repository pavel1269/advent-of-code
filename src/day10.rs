
pub fn get_part1_result() -> i64 {
    let adapters = get_challenge_input();
    let result = get_part1_result_adapters(&adapters);
    return result;
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
    fn example_get_jolt_differences() {
        let adapters = get_example();
        let result = get_jolt_differences(&adapters);

        assert_eq!([7, 0, 5], result);
    }

    #[test]
    fn example_get_part1_res() {
        let adapters = get_example();
        let result = get_part1_result_adapters(&adapters);

        assert_eq!(35, result);
    }

    #[test]
    fn example2_get_jolt_differences() {
        let adapters = get_example2();
        let result = get_jolt_differences(&adapters);

        assert_eq!([22, 0, 10], result);
    }

    #[test]
    fn input_get_part1_res() {
        let result = get_part1_result();

        assert_eq!(2346, result);
    }
}
