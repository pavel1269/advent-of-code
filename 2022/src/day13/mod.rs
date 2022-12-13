use std::{str::Chars, cmp::Ordering};

pub fn get_solution_part1() -> String {
    let input = get_input();
    let result = get_packet_indice(input);
    return result.to_string();
}

pub fn get_solution_part2() -> String {
    let input = get_input();
    let result = order_packets_dividers(input);
    return result.to_string();
}

fn order_packets_dividers(input: &str) -> usize {
    let packets = parse_input(input);
    let dividers = get_dividers();
    let dividers = parse_input(dividers);
    
    let mut all_packets = packets.iter().cloned().map(|(p, _)| p).collect::<Vec<Packet>>();
    all_packets.append(&mut packets.iter().cloned().map(|(_, p)| p).collect::<Vec<Packet>>());
    all_packets.push(dividers[0].0.clone());
    all_packets.push(dividers[0].1.clone());
    all_packets.sort();

    let div1_pos = all_packets.iter().position(|p| p == &dividers[0].0.clone()).unwrap();
    let div2_pos = all_packets.iter().position(|p| p == &dividers[0].1.clone()).unwrap();

    return (div1_pos + 1) * (div2_pos + 1);
}

fn get_packet_indice(input: &str) -> usize {
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Packet {
    values: Vec<PacketValue>,
}

impl Packet {
    fn from_int_value(value: i32) -> Packet {
        Packet { values: vec![PacketValue::Integer(value)], }
    }
}

impl Ord for Packet {
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

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Packet {}
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
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

fn get_dividers() -> &'static str {
    "[[2]]
[[6]]"
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
        let result = get_packet_indice(input);

        assert_eq!(result, 13);
    }

    #[test]
    fn part1_input() {
        let result = get_solution_part1();

        assert_eq!(result, "5623");
    }

    #[test]
    fn part2_example() {
        let input = get_example_input();
        let result = order_packets_dividers(input);

        assert_eq!(result, 140);
    }
}
