use std::num::ParseIntError;

pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let departure = get_earliest_bus(input);

    return ((departure.2 - departure.0) * departure.1) as i64;
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let departure = get_timestamp_departures_line(input);

    return departure;
}

struct Timetable {
    timestamp: u32,
    buses: Vec<Result<u32, ParseIntError>>,
}

fn get_timestamp_departures_line(input: &str) -> i64 {
    let timetable = parse_input(input);

    // 7,10 -> 49 + 70n
    // 10,7 -> 20 + 70n
    // 7,9 -> 35 + 63n
    // 9,7 -> 27 + 63n
    // 9,10 -> 9 + 90n
    // 10,9 -> 80 + 90n
    // 7,10,9 -> 49 + 70n, 9 -> 259 + 630n

    let mut number: i64 = 0;
    let lcm = match timetable.buses[0] {
        Ok(number) => number as i64,
        _ => panic!("Did not expect first bus to be ignored"),
    };
    let mut lcm = lcm as i64;
    let mut index: usize = 1;

    while index < timetable.buses.len() {
        let bus = match timetable.buses[index] {
            Ok(bus) => bus as i64,
            _ => {
                index += 1;
                continue;
            },
        };

        // println!("Bus: {}, Number: {}, LCM: {}, index: {}", bus, number, lcm, index);

        loop {
            number += lcm;
            if (number + index as i64) % bus == 0 {
                lcm *= bus;
                break;
            }
        }

        index += 1;
    }

    return number;
}

fn get_earliest_bus(input: &str) -> (u32, u32, u32) {
    let timetable = parse_input(input);

    let mut next_departures = timetable
        .buses
        .iter()
        .filter(|entry| match entry {
            Ok(_) => true,
            _ => false,
        })
        .map(|entry| {
            let bus = match entry {
                Ok(bus) => *bus,
                _ => panic!("Cannot fail here"),
            };
            let times = timetable.timestamp / bus;
            let mut next_time = bus * times;
            if next_time < timetable.timestamp {
                next_time += bus;
            }
            // println!("Bus {} next time {}", bus, next_time);
            return (bus, next_time);
        })
        .collect::<Vec<(u32, u32)>>();
    next_departures.sort_by(|a, b| a.1.cmp(&b.1));

    return (timetable.timestamp, next_departures[0].0, next_departures[0].1);
}

fn parse_input(input: &str) -> Timetable {
    let lines = input.lines().collect::<Vec<&str>>();

    if lines.len() != 2 {
        panic!("Unexpected input.")
    }

    let timestamp = lines[0].parse::<u32>().unwrap();
    let buses = lines[1]
        .split(",")
        .map(|bus| bus.parse::<u32>())
        .collect::<Vec<Result<u32, ParseIntError>>>();

    return Timetable {
        timestamp: timestamp,
        buses: buses,
    };
}

fn get_challenge_input() -> &'static str {
    include_str!("./inputs/day13.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
"939
7,13,x,x,59,x,31,19"
    }

    #[test]
    fn example_get_earliest_bus() {
        let input = get_example_input();
        let departure = get_earliest_bus(input);

        assert_eq!((939, 59, 944), departure);
    }

    #[test]
    fn input_part1_result() {
        let result = get_part1_result();

        assert_eq!(115, result);
    }

    #[test]
    fn numbers_7_10_get_timestamp_departures_line() {
        let departure = get_timestamp_departures_line("0\n7,10");
        assert_eq!(49, departure);
    }
    #[test]
    fn numbers_10_7_get_timestamp_departures_line() {
        let departure = get_timestamp_departures_line("0\n10,7");
        assert_eq!(20, departure);
    }
    #[test]
    fn numbers_7_9_get_timestamp_departures_line() {
        let departure = get_timestamp_departures_line("0\n7,9");
        assert_eq!(35, departure);
    }
    #[test]
    fn numbers_10_9_get_timestamp_departures_line() {
        let departure = get_timestamp_departures_line("0\n10,9");
        assert_eq!(80, departure);
    }
    #[test]
    fn numbers_9_10_get_timestamp_departures_line() {
        let departure = get_timestamp_departures_line("0\n9,10");
        assert_eq!(9, departure);
    }
    #[test]
    fn numbers_7_10_9_get_timestamp_departures_line() {
        let departure = get_timestamp_departures_line("0\n7,10,9");
        assert_eq!(259, departure);
    }

    #[test]
    fn example_get_timestamp_departures_line() {
        let input = get_example_input();
        let departure = get_timestamp_departures_line(input);

        assert_eq!(1068781, departure);
    }

    #[test]
    fn example2_get_timestamp_departures_line() {
        let departure = get_timestamp_departures_line("0\n17,x,13,19");

        assert_eq!(3417, departure);
    }

    #[test]
    fn example3_get_timestamp_departures_line() {
        let departure = get_timestamp_departures_line("0\n67,7,59,61");

        assert_eq!(754018, departure);
    }

    #[test]
    fn example4_get_timestamp_departures_line() {
        let departure = get_timestamp_departures_line("0\n67,x,7,59,61");

        assert_eq!(779210, departure);
    }

    #[test]
    fn example5_get_timestamp_departures_line() {
        let departure = get_timestamp_departures_line("0\n67,7,x,59,61");

        assert_eq!(1261476, departure);
    }

    #[test]
    fn example6_get_timestamp_departures_line() {
        let departure = get_timestamp_departures_line("0\n1789,37,47,1889");

        assert_eq!(1202161486, departure);
    }

    #[test]
    fn input_part2_result() {
        let result = get_part2_result();

        assert_eq!(756261495958122, result);
    }
}
