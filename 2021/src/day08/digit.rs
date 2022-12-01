const SIGNALS: u8 = 7;
pub const MASK_CLEAN: u8 = 0x7f;

pub struct Digit {
    signals: u8,
    digit: Option<u8>,
}

impl Digit {
    pub fn get_digit(&self) -> u8 {
        return self.digit.unwrap();
    }

    pub fn get_signals(&self) -> u8 {
        return self.signals;
    }

    pub fn is_any_digit(&self) -> bool {
        return match self.digit {
            None => false,
            Some(_) => true,
        };
    }

    pub fn is_digit(&self, digit: u8) -> bool {
        return match self.digit {
            None => false,
            Some(digit_self) => digit == digit_self,
        };
    }

    pub fn mark_as_digit(&mut self, digit: u8) {
        self.digit = Some(digit);
    }

    pub fn from(signals: &String) -> Digit {
        let mut signals_binary = 0;
        for i in 0..SIGNALS {
            let signal_character = ('a' as u8 + i) as char;
            if signals.contains(signal_character) {
                signals_binary |= 0b1 << i;
            }
        }

        let digit = match signals.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        };

        let digit = Digit {
            signals: signals_binary,
            digit: digit,
        };
        return digit;
    }
}

impl std::fmt::Debug for Digit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        formatter
            .debug_struct("Digit")
            .field("signals", &format!("{:#09b}", self.signals))
            .field("digit", &format!("{:?}", self.digit))
            .finish()
    }
}
