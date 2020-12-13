
pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let departure = get_earliest_bus(input);

    return ((departure.2 - departure.0) * departure.1) as i64;
}

struct Timetable {
    timestamp: u32,
    buses: Vec<u32>,
}

fn get_earliest_bus(input: &str) -> (u32, u32, u32) {
    let timetable = parse_input(input);

    let mut next_departures = timetable
        .buses
        .iter()
        .map(|bus| {
            let times = timetable.timestamp / bus;
            let mut next_time = bus * times;
            if next_time < timetable.timestamp {
                next_time += bus;
            }
            // println!("Bus {} next time {}", bus, next_time);
            return (*bus, next_time);
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
        .filter(|entry| *entry != "x")
        .map(|bus| bus.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

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
}
