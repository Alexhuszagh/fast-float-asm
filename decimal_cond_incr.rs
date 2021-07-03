#![allow(unused_unsafe, dead_code)]

#[derive(Clone)]
pub struct Decimal {
    /// The number of significant digits in the decimal.
    pub num_digits: usize,
    /// The offset of the decimal point in the significant digits.
    pub decimal_point: i32,
    /// If the sign of the float is negative.
    pub negative: bool,
    /// If the number of significant digits stored in the decimal is truncated.
    pub truncated: bool,
    /// Buffer of the raw digits, in the range [0, 9].
    pub digits: [u8; Self::MAX_DIGITS],
}

impl Decimal {
    pub const MAX_DIGITS: usize = 768;

    /// Append a digit to the buffer.
    pub fn try_add_digit(&mut self, digit: u8) {
        if self.num_digits < Self::MAX_DIGITS {
            self.digits[self.num_digits] = digit;
            self.num_digits += 1;
        }
    }
}

pub fn main() {}
