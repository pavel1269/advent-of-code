
pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let parsed_boardpasses = parse_boardpasses(&input);
    let id = get_highest_id_passport(&parsed_boardpasses);

    return id;
}

fn get_highest_id_passport(boardpasses: &Vec<(u32, u8, u8)>) -> i64 {
    let mut highest_id: u32 = 0;
    for boardpass in boardpasses.iter() {
        if boardpass.0 > highest_id {
            highest_id = boardpass.0;
        }
    }

    return highest_id as i64;
}

fn parse_boardpasses(boardpasses: &Vec<&str>) -> Vec<(u32, u8, u8)> {
    let mut parsed_boardpasses: Vec<(u32, u8, u8)> = Vec::new();
    for boardpass in boardpasses.iter() {
        let parsed_boardpass = parse_boardpass(boardpass);
        parsed_boardpasses.push(parsed_boardpass);
    }

    return parsed_boardpasses;
}

fn parse_boardpass(boardpass: &str) -> (u32, u8, u8) {
    if boardpass.len() != 10 {
        panic!("Unexpected length of a boardpass");
    }

    let boardpass = boardpass
        .replace('F', "0")
        .replace('B', "1")
        .replace('L', "0")
        .replace('R', "1");

    let row = u8::from_str_radix(&boardpass[0..7], 2).expect("Could not parse row");
    let column = u8::from_str_radix(&boardpass[7..10], 2).expect("Could not parse column");
    let id = ((row as u32) << 3) + column as u32;

    return (id, row, column);
}

fn get_challenge_input() -> Vec<&'static str> {
    include_str!("./inputs/day05.txt").lines().collect()
}

const EXAMPLE_BOARDPASS: &'static str = "FBFBBFFRLR";

fn get_example() -> Vec<&'static str> {
    vec![
        "BFFFBBFRRR",
        "FFFBBBFRRR",
        "BBFFBBFRLL",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_example_parse_boardpass_correct() {
        let parsed_boardpass = parse_boardpass(EXAMPLE_BOARDPASS);

        assert_eq!((357, 44, 5), parsed_boardpass);
    }

    #[test]
    fn example_part1_correct() {
        let parsed_boardpasses = parse_boardpasses(&get_example());
        let id = get_highest_id_passport(&parsed_boardpasses);

        assert_eq!(820, id);
    }

    #[test]
    fn input_part1_correct() {
        let input = get_challenge_input();
        let parsed_boardpasses = parse_boardpasses(&input);
        let id = get_highest_id_passport(&parsed_boardpasses);

        assert_eq!(938, id);
    }
}
