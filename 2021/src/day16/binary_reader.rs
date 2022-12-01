use num::{NumCast, Num};

pub struct BinaryReader {
    bytes: Vec<u8>,
    bits_read: usize,
}

impl BinaryReader {
    pub fn from(input: &str) -> BinaryReader {
        let input = input.trim();
        let mut input_str;
        let input = if input.len() % 2 == 0 {
            input
        } else {
            input_str = String::from(input);
            input_str.push('0');
            input_str.as_str()
        };

        let bytes = hex::decode(input).unwrap();
        return BinaryReader {
            bytes: bytes,
            bits_read: 0,
        };
    }

    pub fn get_read_bits(&self) -> usize {
        self.bits_read
    }

    pub fn take<T: Num + NumCast + std::ops::Shl<Output = T>>(&mut self, bits: usize) -> T {
        const BITS_IN_BYTE: usize = 8;
        let index = self.bits_read / BITS_IN_BYTE;
        let bits_read = self.bits_read % BITS_IN_BYTE;
        let remaining_bits = BITS_IN_BYTE - bits_read;

        if index >= self.bytes.len() {
            panic!("Exceeded reading of bits");
        }

        if remaining_bits == bits {
            let mask = 0xff >> (BITS_IN_BYTE - remaining_bits);
            let result = num::cast(self.bytes[index] & mask).unwrap();
            self.bits_read += bits;
            return result;
        }
        if remaining_bits > bits {
            let dont_read_bits_count = remaining_bits - bits;
            let mask = 0xff >> (BITS_IN_BYTE - remaining_bits + (remaining_bits - bits));
            let result = num::cast((self.bytes[index] >> dont_read_bits_count) & mask).unwrap();
            self.bits_read += bits;
            return result;
        }

        let mask = 0xff >> (BITS_IN_BYTE - remaining_bits);
        let result_part1: T = num::cast(self.bytes[index] & mask).unwrap();

        let left_to_read = bits - remaining_bits;
        self.bits_read += remaining_bits;
        let result_remainder = self.take(left_to_read);
        let result = (result_part1 << num::cast(left_to_read).unwrap()) + result_remainder;

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_byte_unaligned_works() {
        let mut binary = BinaryReader::from("101");

        assert_eq!(1, binary.take::<u8>(4));
        assert_eq!(0, binary.take::<u8>(4));
        assert_eq!(1, binary.take::<u8>(4));
    }

    #[test]
    fn take_byte_works1() {
        let mut binary = BinaryReader::from("1f1055");

        assert_eq!(31, binary.take::<u8>(8));
        assert_eq!(16, binary.take::<u8>(8));
        assert_eq!(85, binary.take::<u8>(8));
    }

    #[test]
    fn take_byte_works2() {
        let mut binary = BinaryReader::from("63b202");

        assert_eq!(99, binary.take::<u8>(8));
        assert_eq!(178, binary.take::<u8>(8));
        assert_eq!(2, binary.take::<u8>(8));
    }

    #[test]
    fn take_bits_from_byte_works1() {
        let mut binary = BinaryReader::from("1f1f");

        assert_eq!(1, binary.take::<u8>(4));
        assert_eq!(3, binary.take::<u8>(2));
        assert_eq!(3, binary.take::<u8>(2));

        assert_eq!(0, binary.take::<u8>(3));
        assert_eq!(1, binary.take::<u8>(1));
        assert_eq!(15, binary.take::<u8>(4));
    }

    #[test]
    fn take_bits_from_byte_works2() {
        let mut binary = BinaryReader::from("bc35c9");

        assert_eq!(5, binary.take::<u8>(3));
        assert_eq!(3, binary.take::<u8>(2));
        assert_eq!(4, binary.take::<u8>(3));

        assert_eq!(6, binary.take::<u8>(5));
        assert_eq!(5, binary.take::<u8>(3));

        assert_eq!(12, binary.take::<u8>(4));
        assert_eq!(2, binary.take::<u8>(2));
        assert_eq!(1, binary.take::<u8>(2));
    }

    #[test]
    fn take_bits_from_different_words_works() {
        let mut binary = BinaryReader::from("c18767ca");

        assert_eq!(24, binary.take::<u8>(5));
        assert_eq!(6, binary.take::<u8>(3 + 2));
        assert_eq!(29, binary.take::<u8>(6 + 2));
        assert_eq!(159, binary.take::<u8>(6 + 2));
    }

    #[test]
    fn take_bits_from_multiple_words_works() {
        let mut binary = BinaryReader::from("17282CDC6200AE");

        assert_eq!(5, binary.take::<u8>(6));

        assert_eq!(827571, binary.take::<u32>(2 + 8 + 8 + 2));
        assert_eq!(3633, binary.take::<u16>(6 + 7));
        assert_eq!(174, binary.take::<u32>(1 + 8 + 8));
    }
}
