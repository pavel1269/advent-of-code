
pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let result = get_scanning_error_rate(input);
    return result;
}

#[derive(Debug)]
struct Restriction {
    name: String,
    range1: (i32, i32),
    range2: (i32, i32),
}

struct Input {
    restrictions: Vec<Restriction>,
    tickets: Vec<Vec<i32>>,
}

fn get_scanning_error_rate(input: &str) -> i64 {
    let input = parse_input(input);
    
    let mut error_rate: i64 = 0;
    for ticket in input.tickets {
        for entry in ticket.iter() {
            let valid = input.restrictions.iter().any(|restriction| {
                (*entry >= restriction.range1.0 && *entry <= restriction.range1.1) || (*entry >= restriction.range2.0 && *entry <= restriction.range2.1)
            });
            if !valid {
                error_rate += *entry as i64;
            }
        }
    }

    return error_rate;
}

fn parse_input(input: &str) -> Input {
    let part2_start = "your ticket:";
    let part3_start = "nearby tickets:";

    let part2_start_index = input.find(part2_start).unwrap();
    let part3_start_index = input.find(part3_start).unwrap();

    let part1 = input[..part2_start_index].trim();
    let part3 = input[part3_start_index + part3_start.len()..].trim();

    let restrictions = parse_restrictions(part1);
    // ignore my ticket
    let tickets = parse_tickets(part3, restrictions.len());

    return Input {
        restrictions: restrictions,
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
            range1: (
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
            ),
            range2: (
                captures[4].parse().unwrap(),
                captures[5].parse().unwrap(),
            ),
        };

        restrictions.push(restriction);
    }

    return restrictions;
}

fn get_challenge_input() -> &'static str {
    include_str!("input.txt")
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
}
