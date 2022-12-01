const MOD: u64 = 20201227;
const SUBJECT_NUMBER: u64 = 7;

pub fn get_part1_result() -> i64 {
    let mut input = get_challenge_input();
    let result = find_encryption_key(&mut input);
    return result as i64;
}

fn find_encryption_key(input: &mut Vec<u64>) -> u64 {
    let mut loop1: u64 = 0;
    let mut num2: u64 = 0;

    let mut index: u64 = 0;
    let mut encryption_key = 1;
    loop {
        index += 1;
        encryption_key = crypto_loop(encryption_key, SUBJECT_NUMBER);

        // println!("[{}] = {}", index, encryption_key);
        if input.contains(&encryption_key) {
            if loop1 == 0 {
                loop1 = index;
            } else {
                num2 = encryption_key;
            }

            if input[0] == encryption_key {
                input.remove(0);
            } else {
                input.remove(1);
            }
        }

        if input.len() == 0 {
            break;
        }
    }

    encryption_key = 1;
    for _index in 0..loop1 {
        //println!("[{}] = {}", _index, encryption_key);
        encryption_key = crypto_loop(encryption_key, num2);
    }

    return encryption_key;
}

fn crypto_loop(value: u64, subject_number: u64) -> u64 {
    let value = value * subject_number;
    let value = value % MOD;
    return value;
}

fn get_challenge_input() -> Vec<u64> {
    vec![16616892, 14505727]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> Vec<u64> {
        vec![5764801, 17807724]
    }

    #[test]
    fn example_sth_result() {
        let mut input = get_example_input();
        let result = find_encryption_key(&mut input);

        assert_eq!(14897079, result);
    }

    #[test]
    fn input_part1_result() {
        let result = get_part1_result();

        assert_eq!(4441893, result);
    }
}
