use self::binary_reader::BinaryReader;

pub mod binary_reader;

pub fn get_solution_part1() -> i64 {
    let input = get_input();
    let packet = Packet::from(input);
    let result = packet.version_sum();
    return result;
}

pub fn get_solution_part2() -> i64 {
    let input = get_input();
    let packet = Packet::from(input);
    let result = packet.reduce();
    return result;
}

struct Packet {
    version: u8,
    type_id: u8,
    sub_packets: Vec<Packet>,
    number: Option<i64>,
}

impl Packet {
    pub fn version_sum(&self) -> i64 {
        let sum: i64 = self.sub_packets.iter().map(|p| p.version_sum()).sum();
        return sum + self.version as i64;
    }

    pub fn reduce(&self) -> i64 {
        if self.type_id == 4 {
            return self.number.unwrap();
        }

        let mut operands = self.sub_packets.iter().map(|p| p.reduce());
        let result: i64 = match self.type_id {
            0 => operands.sum(),
            1 => operands.reduce(|a, b| a * b).unwrap(),
            2 => operands.min().unwrap(),
            3 => operands.max().unwrap(),
            5 | 6 | 7 => {
                assert_eq!(2, operands.len());
                let operand1 = operands.next().unwrap();
                let operand2 = operands.next().unwrap();
                let result = match self.type_id {
                    5 => operand1 > operand2,
                    6 => operand1 < operand2,
                    7 => operand1 == operand2,
                    _ => panic!(),
                };

                return if result { 1 } else { 0 };
            },
            _ => todo!(),
        };

        return result;
    }

    pub fn from(input: &str) -> Packet {
        let mut binary_reader = binary_reader::BinaryReader::from(input);
        let packet = Packet::parse_packet(&mut binary_reader);
        return packet;
    }

    fn parse_packet(binary_reader: &mut BinaryReader) -> Packet {
        let version = binary_reader.take::<u8>(3);
        let type_id = binary_reader.take::<u8>(3);

        let mut number = None;
        let mut sub_packets = Vec::new();
        match type_id {
            4 => {
                let mut is_not_last = true;
                let mut read_number = 0;
                while is_not_last {
                    is_not_last = binary_reader.take::<u8>(1) == 1;
                    read_number <<= 4;
                    let number_part = binary_reader.take::<i64>(4);
                    read_number += number_part;
                }
                number = Some(read_number);
            },
            _ => {
                let number_of_packets = binary_reader.take::<u8>(1) == 1;
                if number_of_packets {
                    let size = binary_reader.take::<usize>(11);
                    
                    for _index in 0..size {
                        let sub_packet = Packet::parse_packet(binary_reader);
                        sub_packets.push(sub_packet);
                    }
                } else {
                    let size = binary_reader.take::<usize>(15);
                    let mut read = binary_reader.get_read_bits();
                    let boundary = read + size;
                    while read < boundary {
                        let sub_packet = Packet::parse_packet(binary_reader);
                        sub_packets.push(sub_packet);
                        read = binary_reader.get_read_bits();
                    }
                }
            },
        }

        // println!("version: {}, type_id {}, value: {:?}", version, type_id, number);

        return Packet {
            version: version,
            type_id: type_id,
            sub_packets: sub_packets,
            number: number,
        };
    }
}

fn get_input() -> &'static str {
    return include_str!("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_1_input() -> &'static str {
        return "D2FE28";
    }

    fn get_example_2_input() -> &'static str {
        return "38006F45291200";
    }

    fn get_example_3_input() -> &'static str {
        return "EE00D40C823060";
    }

    fn get_example_4_input() -> &'static str {
        return "8A004A801A8002F478";
    }

    fn get_example_5_input() -> &'static str {
        return "620080001611562C8802118E34";
    }

    fn get_example_6_input() -> &'static str {
        return "C0015000016115A2E0802F182340";
    }

    fn get_example_7_input() -> &'static str {
        return "A0016C880162017C3686B18A3D4780";
    }

    macro_rules! test_part1 {
        ($name: ident, $input: expr, $expectation: expr) => {
            #[test]
            fn $name() {
                let packet = Packet::from($input);
                let result = packet.version_sum();
                assert_eq!($expectation, result);
            }
        };
    }

    test_part1!(example1_part1_correct_result, get_example_1_input(), 6);
    test_part1!(example2_part1_correct_result, get_example_2_input(), 9);
    test_part1!(example3_part1_correct_result, get_example_3_input(), 14);
    test_part1!(example4_part1_correct_result, get_example_4_input(), 16);
    test_part1!(example5_part1_correct_result, get_example_5_input(), 12);
    test_part1!(example6_part1_correct_result, get_example_6_input(), 23);
    test_part1!(example7_part1_correct_result, get_example_7_input(), 31);

    #[test]
    fn input_part1_correct_result() {
        let result = get_solution_part1();

        assert_eq!(936, result);
    }
    
    macro_rules! test_part2 {
        ($name: ident, $input: expr, $expectation: expr) => {
            #[test]
            fn $name() {
                let packet = Packet::from($input);
                let result = packet.reduce();
                assert_eq!($expectation, result);
            }
        };
    }

    test_part2!(example1_part2_correct_result, get_example_1_input(), 2021);
    test_part2!(example2_part2_correct_result, get_example_2_input(), 1);
    test_part2!(example3_part2_correct_result, get_example_3_input(), 3);
    test_part2!(example4_part2_correct_result, get_example_4_input(), 15);
    test_part2!(example5_part2_correct_result, get_example_5_input(), 46);
    test_part2!(example6_part2_correct_result, get_example_6_input(), 46);
    test_part2!(example7_part2_correct_result, get_example_7_input(), 54);
    
    test_part2!(example8_part2_correct_result, "C200B40A82", 3);
    test_part2!(example9_part2_correct_result, "04005AC33890", 54);
    test_part2!(example10_part2_correct_result, "880086C3E88112", 7);
    test_part2!(example11_part2_correct_result, "CE00C43D881120", 9);
    test_part2!(example12_part2_correct_result, "D8005AC2A8F0", 1);
    test_part2!(example13_part2_correct_result, "F600BC2D8F", 0);
    test_part2!(example14_part2_correct_result, "9C005AC2F8F0", 0);
    test_part2!(example15_part2_correct_result, "9C0141080250320F1802104A08", 1);

    #[test]
    fn input_part2_correct_result() {
        let result = get_solution_part2();

        assert_eq!(6802496672062, result);
    }
}
