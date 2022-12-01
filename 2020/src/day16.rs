pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = get_scanning_error_rate(input);
    return result;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();

    let mut input = parse_input(input);
    identify_ticket_fields(&mut input);

    let mut multiply: i64 = 1;
    for index in input
        .restrictions
        .iter()
        .filter(|restruction| restruction.name.starts_with("departure"))
        .map(|restriction| restriction.index)
    {
        multiply *= input.my_ticket[index] as i64;
    }

    return multiply;
}

#[derive(Debug)]
struct Restriction {
    name: String,
    index: usize,
    range1: (i32, i32),
    range2: (i32, i32),
}

struct Input {
    restrictions: Vec<Restriction>,
    my_ticket: Vec<i32>,
    tickets: Vec<Vec<i32>>,
}

fn identify_ticket_fields(input: &mut Input) {
    input.tickets = input
        .tickets
        .iter()
        .cloned()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|value| is_ticket_value_valid_any_restriction(*value, &input.restrictions))
        })
        .collect::<Vec<Vec<i32>>>();

    let restrictions_count: usize = input.restrictions.len();
    let mut possibilities: Vec<Vec<usize>> = Vec::new();
    for (restriction_index, restriction) in input.restrictions.iter_mut().enumerate() {
        possibilities.push(Vec::new());
        let mut column_found = false;
        for available_column in 0..restrictions_count {
            let mut valid = true;
            for ticket in input.tickets.iter() {
                if !is_ticket_value_valid_restriction(ticket[available_column], restriction) {
                    valid = false;
                    break;
                }
            }

            if valid {
                possibilities[restriction_index].push(available_column);
                column_found = true;
            }
        }

        if !column_found {
            panic!(format!(
                "Could not find any column for retriction '{}'",
                restriction.name
            ));
        }
    }

    // println!("{:?}", possibilities);
    // [
    //     [1, 4, 5, 8, 9, 11, 12, 13, 14, 16, 17, 18],
    //     [1, 2, 4, 5, 8, 9, 11, 12, 13, 14, 16, 17, 18],
    //     [1, 4, 8, 9, 11, 12, 13, 14, 17, 18],
    //     [1, 4, 5, 8, 9, 11, 12, 13, 14, 17, 18],
    //     [4, 8, 9, 11, 12, 13, 14, 17, 18],
    //     [4, 8, 9, 11, 12, 13, 14, 18],
    //     [1, 2, 4, 5, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
    //     [1, 2, 4, 5, 8, 9, 10, 11, 12, 13, 14, 16, 17, 18],
    //     [9],
    //     [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    //     [1, 2, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
    //     [4, 8, 9, 12, 18],
    //     [4, 8, 9, 12, 14, 18],
    //     [1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    //     [4, 8, 9, 12],
    //     [8, 9],
    //     [4, 8, 9, 11, 12, 14, 18],
    //     [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19],
    //     [8, 9, 12],
    //     [1, 2, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    // ]
    // => 9, 8, 12, 4, 18, 14, 11, 13, 17, 1, 5, 16, 2, 10, 15, 6, 19, 3, 7, 0

    let mut possibilities = possibilities
        .iter_mut()
        .enumerate()
        .collect::<Vec<(usize, &mut Vec<usize>)>>();
    loop {
        // println!("{:?}", possibilities);
        // println!("{:?}", input.restrictions.iter().map(|res| res.index).collect::<Vec<usize>>());

        let mut possibility = possibilities
            .iter_mut()
            .enumerate()
            .filter(|possibility| possibility.1 .1.len() == 1);
        let (possibility_index, possibility) = possibility.next().unwrap();

        let restriction_index = *possibility.1.first().unwrap();
        input.restrictions[possibility.0].index = restriction_index;

        possibilities.remove(possibility_index);
        if possibilities.len() == 0 {
            break;
        }

        for possibility in possibilities.iter_mut() {
            let index = possibility
                .1
                .iter()
                .copied()
                .enumerate()
                .find(|(_, possibility)| *possibility == restriction_index);

            if index.is_some() {
                let index = index.unwrap();
                possibility.1.remove(index.0);
            }
        }
    }
}

fn get_scanning_error_rate(input: &str) -> i64 {
    let input = parse_input(input);

    let mut error_rate: i64 = 0;
    for ticket in input.tickets {
        for value in ticket.iter() {
            if !is_ticket_value_valid_any_restriction(*value, &input.restrictions) {
                error_rate += *value as i64;
            }
        }
    }

    return error_rate;
}

fn is_ticket_value_valid_any_restriction(value: i32, restrictions: &Vec<Restriction>) -> bool {
    restrictions
        .iter()
        .any(|restriction| is_ticket_value_valid_restriction(value, restriction))
}

fn is_ticket_value_valid_restriction(value: i32, restriction: &Restriction) -> bool {
    (value >= restriction.range1.0 && value <= restriction.range1.1)
        || (value >= restriction.range2.0 && value <= restriction.range2.1)
}

fn parse_input(input: &str) -> Input {
    let part2_start = "your ticket:";
    let part3_start = "nearby tickets:";

    let part2_start_index = input.find(part2_start).unwrap();
    let part3_start_index = input.find(part3_start).unwrap();

    let part1 = input[..part2_start_index].trim();
    let part2 = input[part2_start_index + part2_start.len()..part3_start_index].trim();
    let part3 = input[part3_start_index + part3_start.len()..].trim();

    let restrictions = parse_restrictions(part1);
    let my_ticket = parse_ticket(part2, restrictions.len());
    let tickets = parse_tickets(part3, restrictions.len());

    return Input {
        restrictions: restrictions,
        my_ticket: my_ticket,
        tickets: tickets,
    };
}

fn parse_tickets(input: &str, length: usize) -> Vec<Vec<i32>> {
    let input_lines = input.lines().collect::<Vec<&str>>();
    let mut tickets: Vec<Vec<i32>> = Vec::new();
    for ticket_str in input_lines.iter() {
        let ticket = parse_ticket(ticket_str, length);
        tickets.push(ticket);
    }
    return tickets;
}

fn parse_ticket(input: &str, length: usize) -> Vec<i32> {
    let mut ticket: Vec<i32> = Vec::new();
    for field in input.split(",") {
        ticket.push(field.parse::<i32>().unwrap());
    }
    if ticket.len() != length {
        panic!("Unexpected input format");
    }
    return ticket;
}

fn parse_restrictions(input: &str) -> Vec<Restriction> {
    use regex::Regex;
    let regex = Regex::new("^([^:]+): (\\d+)-(\\d+) or (\\d+)-(\\d+)$").unwrap();

    let mut restrictions: Vec<Restriction> = Vec::new();
    for line in input.lines() {
        let captures = regex.captures(line).unwrap();

        let restriction = Restriction {
            name: captures[1].to_string(),
            index: 0,
            range1: (captures[2].parse().unwrap(), captures[3].parse().unwrap()),
            range2: (captures[4].parse().unwrap(), captures[5].parse().unwrap()),
        };

        restrictions.push(restriction);
    }

    return restrictions;
}

fn get_challenge_input() -> &'static str {
    include_str!("inputs/day16.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example() -> &'static str {
        "class: 1-3 or 5-7
row: 6-11 or 33-44
seat number: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
    }

    #[test]
    fn example_parse_restrictions_parsed() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat number: 13-40 or 45-50";
        let result = parse_restrictions(input);

        assert_eq!(3, result.len());
    }

    #[test]
    fn example_get_scanning_error_rate() {
        let input = get_example();
        let result = get_scanning_error_rate(input);

        assert_eq!(71, result);
    }

    #[test]
    fn input_get_part1_result() {
        let result = get_part1_result();

        assert_eq!(25961, result);
    }

    #[test]
    fn input_get_part2_result() {
        let result = get_part2_result();

        assert_eq!(603409823791, result);
    }
}
