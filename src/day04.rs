
use std::collections::HashMap;

pub fn get_solution_part1() -> i64 {
    let input = get_challenge_input();
    let passports = parse_input(input);
    let result = filter_valid_passports_fields(passports).len();

    return result as i64;
}

pub fn get_solution_part2() -> i64 {
    let input = get_challenge_input();
    let passports = parse_input(input);
    let valid_passports = filter_valid_passports_fields(passports);
    let result = count_passports_values_valid(&valid_passports);

    return result;
}

fn count_passports_values_valid(passports: &Vec<Vec<(String, String)>>) -> i64 {
    let mut valid_count = 0;

    for passport in passports.iter() {
        let mut passport_table: HashMap<String, String> = HashMap::new();
        for passport_field in passport.iter() {
            passport_table.insert(passport_field.0.clone(), passport_field.1.clone());
        }
        
        if validate_passport_values(&passport_table) {
            valid_count += 1;
        }
    }

    return valid_count;
}

fn validate_passport_values(passport: &HashMap<String, String>) -> bool {

    match passport.get("byr") {
        None => return false,
        Some(value) => {
            if !is_value_within_range(&value, 1920, 2002) {
                return false;
            }
        },
    }

    match passport.get("iyr") {
        None => return false,
        Some(value) => {
            if !is_value_within_range(&value, 2010, 2020) {
                return false;
            }
        },
    }

    match passport.get("eyr") {
        None => return false,
        Some(value) => {
            if !is_value_within_range(&value, 2020, 2030) {
                return false;
            }
        },
    }

    match passport.get("hgt") {
        None => return false,
        Some(value) => {
            let height_unit = value.chars().rev().take(2).collect::<Vec<char>>().iter().rev().collect::<String>();
            let height: String = value.chars().take(value.len() - 2).collect();
            match height_unit.as_str() {
                "cm" => {
                    if !is_value_within_range(&height, 150, 193) {
                        return false;
                    }
                }
                "in" => {
                    if !is_value_within_range(&height, 59, 76) {
                        return false;
                    }
                }
                _ => return false,
            }
        },
    }

    use regex::Regex;
    match passport.get("hcl") {
        None => return false,
        Some(value) => {
            let regex = Regex::new("^#[a-z0-9]{6}$").unwrap();
            if !regex.is_match(value) {
                return false;
            }
        },
    }

    match passport.get("ecl") {
        None => return false,
        Some(value) => {
            let regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            if !regex.is_match(value) {
                return false;
            }
        },
    }

    match passport.get("pid") {
        None => return false,
        Some(value) => {
            let regex = Regex::new("^\\d{9}$").unwrap();
            if !regex.is_match(value) {
                return false;
            }
        },
    }

    return true;
}

fn is_value_within_range(value: &String, min: i32, max: i32) -> bool {
    match value.parse::<i32>() {
        Err(_) => return false,
        Ok(value) => {
            return (value >= min) & (value <= max);
        },
    }
}

fn filter_valid_passports_fields(passports: Vec<String>) -> Vec<Vec<(String, String)>> {
    let mut valid_passports: Vec<Vec<(String, String)>> = vec!();
    let required_fields = get_passport_required_fields();
    let required_fields_count = required_fields.len();

    for passport in passports {
        let passport_fields: Vec<&str> = passport.split_whitespace().collect();

        if passport_fields.len() < required_fields_count {
            continue;
        }
        
        let passport_values: Vec<(String, String)> =
            passport_fields
                .iter()
                .map(|field| field.splitn(2, ":").collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>()
                .iter()
                .map(|field| (field[0].to_string(), field[1].to_string()))
                .collect();
        
        let is_valid = check_passport(&passport_values, &required_fields);

        if is_valid {
            valid_passports.push(passport_values);
        }
    }

    return valid_passports;
}

fn check_passport(passport_values: &Vec<(String, String)>, required_fields: &Vec<&str>) -> bool {
    for required_field in required_fields.iter() {
        if !passport_values.iter().any(|field| field.0 == *required_field) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_parse_passports_correct_count() {
        let input = get_example_input_part1();
        let passports = parse_input(input);

        assert_eq!(4, passports.len());
    }

    #[test]
    fn example1_part1_correct_result() {
        let input = get_example_input_part1();
        let passports = parse_input(input);
        let result = filter_valid_passports_fields(passports).len();

        assert_eq!(2, result);
    }

    #[test]
    fn input_part1_correct_result() {
        let input = get_challenge_input();
        let passports = parse_input(input);
        let result = filter_valid_passports_fields(passports).len();

        assert_eq!(182, result);
    }

    #[test]
    fn example2_parse_passports_correct_count() {
        let input = get_example_input_part2();
        let passports = parse_input(input);

        assert_eq!(16, passports.len());
    }

    #[test]
    fn example2_part2_correct_result() {
        let input = get_example_input_part2();
        let passports = parse_input(input);
        let valid_passports = filter_valid_passports_fields(passports);
        let result = count_passports_values_valid(&valid_passports);

        assert_eq!(4, result);
    }

    #[test]
    fn input_part2_correct_result() {
        let input = get_challenge_input();
        let passports = parse_input(input);
        let valid_passports = filter_valid_passports_fields(passports);
        let result = count_passports_values_valid(&valid_passports);

        assert_eq!(109, result);
    }

    fn get_example_input_part1() -> &'static str {
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

    fn get_example_input_part2() -> &'static str {
        "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2009 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:194cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:58in hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:158cm hcl:#b6652ah ecl:blu byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:158cm hcl:#b6652aa ecl:blu byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:158cm hcl:#b6652a ecl:red byr:1944 eyr:2021 pid:093154719

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:00093154719

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:09315719

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022"
    }
}
