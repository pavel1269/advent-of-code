use std::{str::Chars, cmp::Ordering};

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = compare_packets(input);
    return result.to_string();
}

fn compare_packets(input: &str) -> usize {
    let packet_pairs = parse_input(input);
    let mut indices = 0;
    for (index, (packet1, packet2)) in packet_pairs.iter().enumerate() {
        if packet1.cmp(&packet2) == Ordering::Less {
            indices += index + 1;
        }
    }
    return indices;
}

fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    let mut lines_iter = input.lines();
    let mut packets = Vec::new();
    while let Some(line) = lines_iter.next() {
        let other = lines_iter.next().unwrap();
        lines_iter.next();

        let packet1 = parse_packet_start(line);
        let packet2 = parse_packet_start(other);
        packets.push((packet1, packet2));
    }

    return packets;
}

#[derive(Debug)]
enum PacketValue {
    Packet(Packet),
    Integer(i32),
}

impl PacketValue {
    fn cmp(&self, other: &PacketValue) -> Ordering {
        match self {
            PacketValue::Integer(int1) => match other {
                PacketValue::Integer(int2) => int1.cmp(int2),
                PacketValue::Packet(packet2) => Packet::from_int_value(*int1).cmp(packet2),
            },
            PacketValue::Packet(packet1) => match other {
                PacketValue::Integer(int2) => packet1.cmp(&Packet::from_int_value(*int2)),
                PacketValue::Packet(packet2) => packet1.cmp(packet2),
            },
        }
    }
}

#[derive(Debug)]
struct Packet {
    values: Vec<PacketValue>,
}

impl Packet {
    fn from_int_value(value: i32) -> Packet {
        Packet { values: vec![PacketValue::Integer(value)], }
    }

    fn cmp(&self, other: &Packet) -> Ordering {
        let mut first_iter = self.values.iter();
        let mut second_iter = other.values.iter();
        loop {
            let first = first_iter.next();
            let second = second_iter.next();

            match first {
                None => match second {
                    None => return Ordering::Equal,
                    Some(_) => return Ordering::Less,
                },
                Some(first) => match second {
                    None => return Ordering::Greater,
                    Some(second) => match first.cmp(second) {
                        Ordering::Equal => {},
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                    },
                },
            }
        }
    }
}

fn parse_packet_start(line: &str) -> Packet {
    let mut chars = line.chars();

    if chars.next().unwrap() != '[' {
        panic!()
    }

    return parse_packet(&mut chars);
}

fn parse_packet(iter: &mut Chars) -> Packet {
    let mut content = Vec::new();
    let mut number = String::new();
    while let Some(char) = iter.next() {
        if char == ']' {
            if number.len() > 0 {
                content.push(PacketValue::Integer(number.parse().unwrap()));
            }
            
            return Packet {
                values: content,
            };
        }
        else if char == '[' {
            content.push(PacketValue::Packet(parse_packet(iter)));
        }
        else if char == ',' {
            if number.len() > 0 {
                content.push(PacketValue::Integer(number.parse().unwrap()));
            }
            number = String::new();
        }
        else {
            if char > '9' || char < '0' {
                panic!();
            }
            number.push(char);
        }
    }

    panic!();
}

fn get_input() -> &'static str {
    include_str!("./input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
    }

    #[test]
    fn part1_example() {
        let input = get_example_input();
        let result = compare_packets(input);

        assert_eq!(result, 13);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "5623");
    }
}
