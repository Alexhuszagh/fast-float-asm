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
        let d =
            // SAFETY: self.len() > u.len(), so [0..u.len()] will always be safe.
            (0..u.len()).fold(0, |d, i| d | unsafe { self.get_unchecked(i) ^ u.get_unchecked(i) });
        d == 0 || d == 32
    }

    /// Get the remaining slice after the first N elements.
    fn advance(&self, n: usize) -> &[u8] {
        &self.as_ref()[n..]
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

impl<'a> AsciiStr<'a> {
    pub fn eq_ignore_case(&self, u: &[u8]) -> bool {
        self.slc.eq_ignore_case(u)
    }

    pub fn is_nan(&self) -> bool {
        if self.slc.len() >= 3 {
            self.eq_ignore_case(b"nan")
        } else {
            false
        }
    }
}

impl<'a> AsRef<[u8]> for AsciiStr<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.slc
    }
}

impl<'a> ByteSlice for AsciiStr<'a> {}

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
}

pub fn main() {}
