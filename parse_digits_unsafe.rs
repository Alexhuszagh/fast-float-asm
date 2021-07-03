#![allow(unused_unsafe, dead_code)]

use std::ptr;

/// Helper methods to process immutable bytes.
pub trait ByteSlice: AsRef<[u8]> {
    unsafe fn get_unchecked(&self, i: usize) -> u8 {
        debug_assert!(self.as_ref().len() > i);
        // SAFETY: safe as long as i <= self.as_ref().len()
        unsafe { *self.as_ref().get_unchecked(i) }
    }

    unsafe fn first_unchecked(&self) -> u8 {
        debug_assert!(!self.is_empty());
        // SAFETY: safe as long as self is not empty
        unsafe { self.get_unchecked(0) }
    }

    /// Get if the slice contains no elements.
    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    /// Check if the slice at least `n` length.
    fn check_len(&self, n: usize) -> bool {
        n <= self.as_ref().len()
    }

    /// Check if the first character in the slice is equal to c.
    fn first_is(&self, c: u8) -> bool {
        self.as_ref().first() == Some(&c)
    }

    /// Check if the first character in the slice is equal to c1 or c2.
    fn first_is2(&self, c1: u8, c2: u8) -> bool {
        if let Some(&c) = self.as_ref().first() {
            c == c1 || c == c2
        } else {
            false
        }
    }

    /// Bounds-checked test if the first character in the slice is a digit.
    fn first_isdigit(&self) -> bool {
        if let Some(&c) = self.as_ref().first() {
            c.is_ascii_digit()
        } else {
            false
        }
    }

    /// Check if self starts with u with a case-insensitive comparison.
    fn eq_ignore_case(&self, u: &[u8]) -> bool {
        debug_assert!(self.as_ref().len() >= u.len());
        let iter = self.as_ref().iter().zip(u.iter());
        let d = iter.fold(0, |i, (&x, &y)| i | (x ^ y));
        d == 0 || d == 32
    }

    /// Get the remaining slice after the first N elements.
    fn advance(&self, n: usize) -> &[u8] {
        &self.as_ref()[n..]
    }

    /// Get the remaining slice after the first N elements.
    unsafe fn advance_unchecked(&self, n: usize) -> &[u8] {
        unsafe { self.as_ref().get_unchecked(n..) }
    }

    /// Get the slice after skipping all leading characters equal c.
    fn skip_chars(&self, c: u8) -> &[u8] {
        let mut s = self.as_ref();
        while s.first_is(c) {
            s = s.advance(1);
        }
        s
    }

    /// Get the slice after skipping all leading characters equal c1 or c2.
    fn skip_chars2(&self, c1: u8, c2: u8) -> &[u8] {
        let mut s = self.as_ref();
        while s.first_is2(c1, c2) {
            s = s.advance(1);
        }
        s
    }

    /// Read 8 bytes as a 64-bit integer in little-endian order.
    unsafe fn read_u64_unchecked(&self) -> u64 {
        debug_assert!(self.check_len(8));
        let src = self.as_ref().as_ptr() as *const u64;
        // SAFETY: safe as long as self is at least 8 bytes
        u64::from_le(unsafe { ptr::read_unaligned(src) })
    }

    /// Try to read the next 8 bytes from the slice.
    fn read_u64(&self) -> Option<u64> {
        if self.check_len(8) {
            // SAFETY: self must be at least 8 bytes.
            Some(unsafe { self.read_u64_unchecked() })
        } else {
            None
        }
    }

    /// Calculate the offset of slice from another.
    unsafe fn offset_from(&self, other: &Self) -> isize {
        // SAFETY: safe as long as self and other are of the same array.
        unsafe { self.as_ref().as_ptr().offset_from(other.as_ref().as_ptr()) }
    }
}

impl ByteSlice for [u8] {}

pub struct AsciiStr<'a> {
    slc: &'a [u8],
}

impl<'a> AsciiStr<'a> {}

impl<'a> AsRef<[u8]> for AsciiStr<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.slc
    }
}

impl<'a> ByteSlice for AsciiStr<'a> {}

fn parse_digits(s: &mut &[u8], mut f: impl FnMut(u8)) {
    while !s.is_empty() {
        // SAFETY: s cannot be empty.
        let c = unsafe { s.first_unchecked() }.wrapping_sub(b'0');
        if c < 10 {
            f(c);
            *s = unsafe { s.advance_unchecked(1) };
        } else {
            break;
        }
    }
}

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
    fn try_add_digit(&mut self, digit: u8) {
        if self.num_digits < Self::MAX_DIGITS {
            self.digits[self.num_digits] = digit;
        }
        self.num_digits += 1;
    }
}

pub fn parse_decimal_digits(mut s: &[u8], d: &mut Decimal) {
    parse_digits(&mut s, |digit| d.try_add_digit(digit));
}

pub fn main() {}
