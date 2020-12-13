
pub fn get_part1_result() -> i64 {
    let input = get_challenge_input();
    let ship = navigate_ship(input);

    return ship.x as i64 + ship.y as i64;
}

fn navigate_ship(input: &str) -> Ship {
    let commands = parse_commands(input);

    let mut ship = Ship {
        x: 0,
        y: 0,
        direction: Direction::East,
    };

    for command in commands.iter() {
        println!("[{}][{}]", ship.x, ship.y);
        match command.direction {
            CommandDirection::Direction(Direction::East) => {
                ship.x += command.value;
            }
            CommandDirection::Direction(Direction::West) => {
                ship.x -= command.value;
            }
            CommandDirection::Direction(Direction::North) => {
                ship.y -= command.value;
            }
            CommandDirection::Direction(Direction::South) => {
                ship.y += command.value;
            }
            CommandDirection::Forward => {
                match ship.direction {
                    Direction::East => {
                        ship.x += command.value;
                    },
                    Direction::West => {
                        ship.x -= command.value;
                    },
                    Direction::North => {
                        ship.y -= command.value;
                    },
                    Direction::South => {
                        ship.y += command.value;
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

struct Ship {
    x: i32,
    y: i32,
    direction: Direction,
}

enum Direction {
    East,
    North,
    West,
    South,
}

enum CommandDirection {
    Direction(Direction),
    Forward,
    Right,
    Left,
}

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

        assert_eq!((17, 8), (ship.x, ship.y));
    }

    #[test]
    fn input_part1_result() {
        let result = get_part1_result();

        assert_eq!(1631, result);
    }
}
