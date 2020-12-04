
pub fn get_solution_part1() -> i64 {
    let input = get_challenge_input();
    let passports = parse_input(input);
    let result = count_valid_passports(passports);

    return result;
}

fn count_valid_passports(passports: Vec<String>) -> i64 {
    let mut valid_count = 0;
    let required_fields = get_passport_required_fields();
    let required_fields_count = required_fields.len();

    for passport in passports {
        let passport_fields: Vec<&str> = passport.split_whitespace().collect();

        if passport_fields.len() < required_fields_count {
            continue;
        }
        
        let passport_values: Vec<Vec<&str>> =
            passport_fields
                .iter()
                .map(|field| field.splitn(2, ":").collect::<Vec<&str>>())
                .collect();
        
        let is_valid = check_passport(&passport_values, &required_fields);

        if is_valid {
            valid_count += 1;
        }
    }

    return valid_count;
}

fn check_passport(passport_values: &Vec<Vec<&str>>, required_fields: &Vec<&str>) -> bool {
    for required_field in required_fields.iter() {
        if !passport_values.iter().any(|field| field[0] == *required_field) {
            return false;
        }
    }

    return true;
}

fn get_passport_required_fields() -> Vec<&'static str> {
    vec![
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
        // "cid",
    ]
}

fn parse_input(input: &str) -> Vec<String> {
    let mut passports: Vec<String> = vec!();
    let mut actual_passport: Vec<&str> = vec!();

    for input_line in input.lines() {
        if input_line.len() == 0 {
            passports.push(actual_passport.join(" "));
            actual_passport.clear();
        } else {
            actual_passport.push(input_line);
        }
    }

    if actual_passport.len() > 0 {
        passports.push(actual_passport.join(" "));
    }

    return passports;
}

fn get_challenge_input() -> &'static str {
    include_str!("./inputs/day04.txt")
}

fn get_example_input() -> &'static str {
    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_parse_passports_correct_count() {
        let input = get_example_input();
        let passports = parse_input(input);

        assert_eq!(4, passports.len());
    }

    #[test]
    fn example_part1_correct_result() {
        let input = get_example_input();
        let passports = parse_input(input);
        let result = count_valid_passports(passports);

        assert_eq!(2, result);
    }

}
