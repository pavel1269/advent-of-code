
pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let ship = navigate_ship(input);

    return i64::abs(ship.coords.x as i64) + i64::abs(ship.coords.y as i64);
}

pub fn get_part2_result() -> i64 {
    let input = get_challenge_input();
    let ship = navigate_with_waypoint(input);

    return i64::abs(ship.coords.x as i64) + i64::abs(ship.coords.y as i64);
}

fn navigate_with_waypoint(input: &str) -> Ship {
    let commands = parse_commands(input);

    let mut ship = Ship {
        coords: Coords {
            x: 0,
            y: 0,
        },
        direction: Direction::East,
    };
    let mut waypoint= Coords {
        x: 10,
        y: -1,
    };

    for command in commands.iter() {
        // println!("Ship: [{}][{}], Waypoint: [{}][{}] -> {:?}", ship.coords.x, ship.coords.y, waypoint.x, waypoint.y, command);
        match command.direction {
            CommandDirection::Direction(Direction::East) => {
                waypoint.x += command.value;
            }
            CommandDirection::Direction(Direction::West) => {
                waypoint.x -= command.value;
            }
            CommandDirection::Direction(Direction::North) => {
                waypoint.y -= command.value;
            }
            CommandDirection::Direction(Direction::South) => {
                waypoint.y += command.value;
            }
            CommandDirection::Forward => {
                ship.coords.x += waypoint.x * command.value;
                ship.coords.y += waypoint.y * command.value;
            },
            // [ 10,  -4] -> [  4,  10] -> [-10,   4] -> [ -4, -10] -> [ 10,  -4]
            // [  a,   b] -> [ -b,   a]
            //               [  c,   d] -> [ -d,   c]
            //                             [  e,   f] -> [ -f,   e]
            //                                           [  g,   h] -> [ -h,   g]
            // [  x,   y] -> [ -y,   x]
            // [  y,  -x] <- [  x,   y]
            CommandDirection::Right => {
                for _ in 0..command.value / 90 {
                    let tmp = waypoint.clone();
                    waypoint.x = -tmp.y;
                    waypoint.y = tmp.x;
                }
            },
            CommandDirection::Left => {
                for _ in 0..command.value / 90 {
                    let tmp = waypoint.clone();
                    waypoint.x = tmp.y;
                    waypoint.y = -tmp.x;
                }
            },
        }
    }

    return ship;
}

fn navigate_ship(input: &str) -> Ship {
    let commands = parse_commands(input);

    let mut ship = Ship {
        coords: Coords {
            x: 0,
            y: 0,
        },
        direction: Direction::East,
    };

    for command in commands.iter() {
        match command.direction {
            CommandDirection::Direction(Direction::East) => {
                ship.coords.x += command.value;
            }
            CommandDirection::Direction(Direction::West) => {
                ship.coords.x -= command.value;
            }
            CommandDirection::Direction(Direction::North) => {
                ship.coords.y -= command.value;
            }
            CommandDirection::Direction(Direction::South) => {
                ship.coords.y += command.value;
            }
            CommandDirection::Forward => {
                match ship.direction {
                    Direction::East => {
                        ship.coords.x += command.value;
                    },
                    Direction::West => {
                        ship.coords.x -= command.value;
                    },
                    Direction::North => {
                        ship.coords.y -= command.value;
                    },
                    Direction::South => {
                        ship.coords.y += command.value;
                    },
                }
            },
            CommandDirection::Left => {
                for _ in 0..command.value / 90 {
                    match ship.direction {
                        Direction::East => {
                            ship.direction = Direction::North;
                        },
                        Direction::North => {
                            ship.direction = Direction::West;
                        },
                        Direction::West => {
                            ship.direction = Direction::South;
                        },
                        Direction::South => {
                            ship.direction = Direction::East;
                        },
                    }
                }
            },
            CommandDirection::Right => {
                for _ in 0..command.value / 90 {
                    match ship.direction {
                        Direction::East => {
                            ship.direction = Direction::South;
                        },
                        Direction::North => {
                            ship.direction = Direction::East;
                        },
                        Direction::West => {
                            ship.direction = Direction::North;
                        },
                        Direction::South => {
                            ship.direction = Direction::West;
                        },
                    }
                }
            },
        }
    }

    return ship;
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
struct Coords {
    x: i32,
    y: i32,
}

struct Ship {
    coords: Coords,
    direction: Direction,
}

#[derive(Debug)]
enum Direction {
    East,
    North,
    West,
    South,
}

#[derive(Debug)]
enum CommandDirection {
    Direction(Direction),
    Forward,
    Right,
    Left,
}

#[derive(Debug)]
struct Command {
    direction: CommandDirection,
    value: i32,
}

fn parse_commands(input: &str) -> Vec<Command> {
    use regex::Regex;
    let regex = Regex::new("^([NSEWLRF])(\\d+)$").unwrap();
    let commands = input.lines().map(|line|
    {
        let captures = regex.captures(line).unwrap();
        let direction = match &captures[1] {
            "N" => CommandDirection::Direction(Direction::North),
            "S" => CommandDirection::Direction(Direction::South),
            "E" => CommandDirection::Direction(Direction::East),
            "W" => CommandDirection::Direction(Direction::West),
            "F" => CommandDirection::Forward,
            "R" => CommandDirection::Right,
            "L" => CommandDirection::Left,
            _ => panic!(format!("Unown direction '{}'", &captures[1])),
        };
        let value = captures[2].parse::<i32>().unwrap();

        Command {
            direction: direction,
            value: value,
        }
    }).collect::<Vec<Command>>();

    return commands;
}

fn get_challenge_input() -> &'static str {
    include_str!("./inputs/day12.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
"F10
N3
F7
R90
F11
"
}

    #[test]
    fn example_navigate_ship_coords_match() {
        let input = get_example_input();
        let ship = navigate_ship(input);

        assert_eq!(Coords { x: 17, y: 8, }, ship.coords);
    }

    #[test]
    fn input_part1_result() {
        let result = get_part1_result();

        assert_eq!(1631, result);
    }

    #[test]
    fn example_navigate_ship_waypoint_coords_match() {
        let input = get_example_input();
        let ship = navigate_with_waypoint(input);

        assert_eq!(Coords { x: 214, y: 72, }, ship.coords);
    }

    #[test]
    fn input_part2_result() {
        let result = get_part2_result();

        assert_eq!(58606, result);
    }
}
