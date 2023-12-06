fn main() {
    let input = get_input();
    let races = parse_input(input);

    let result_part1 = ways_to_win_mult(&races);
    println!("Part1: {}", result_part1);
}

fn ways_to_win_mult(races: &Vec<Race>) -> u32 {
    let mut mult = 1;
    for race in races {
        let ways = ways_to_win(race);
        mult *= ways;
    }
    return mult;
}

fn ways_to_win(race: &Race) -> u32 {
    let is_odd = race.time % 2 == 1;
    let q = race.time / 2;
    let max = if is_odd {
        q * (q + 1)
    } else {
        q * q
    };
    if race.distance >= max {
        return 0
    }

    let mut step = if is_odd {
        2
    } else {
        1
    };
    let mut distance = max;
    let mut ways = 0;
    while distance > race.distance {
        ways += 2;
        distance -= step;
        step += 2;
    }
    if !is_odd {
        ways -= 1;
    }
    return ways;
}

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn parse_input(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<u32> = lines[0].split(' ').skip(1).filter(|str| str.len() > 0).map(|str| str.parse().unwrap()).collect();
    let distances: Vec<u32> = lines[1].split(' ').skip(1).filter(|str| str.len() > 0).map(|str| str.parse().unwrap()).collect();

    if times.len() != distances.len() {
        panic!()
    }

    let mut races = Vec::with_capacity(times.len());
    for index in 0..times.len() {
        let race = Race {
            time: times[index],
            distance: distances[index],
        };
        races.push(race);
    }
    return races;
}

fn get_input() -> &'static str {
    "Time:        62     73     75     65
Distance:   644   1023   1240   1023"
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(7, 5, 6)]
    #[test_case(7, 6, 4)]
    #[test_case(7, 7, 4)]
    #[test_case(7, 8, 4)]
    #[test_case(7, 9, 4)]
    #[test_case(7, 10, 2)]
    #[test_case(7, 11, 2)]
    #[test_case(7, 12, 0)]
    #[test_case(30, 200, 9)]
    fn ways_to_win_test(time: u32, distance: u32, expected: u32) {
        let race = Race{ time, distance };
        let result = ways_to_win(&race);
        assert_eq!(expected, result);
    }

    fn get_example_input() -> &'static str {
        "Time:      7  15   30
Distance:  9  40  200"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let races = parse_input(input);
        let result = ways_to_win_mult(&races);
        assert_eq!(result, 288);
    }
}
