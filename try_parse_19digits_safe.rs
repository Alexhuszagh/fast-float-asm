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
    /// Advance the view by n, advancing it in-place to (n..).
    unsafe fn step_by(&mut self, n: usize) -> &mut Self {
        // SAFETY: safe as long n is less than the buffer length
        self.slc = unsafe { self.slc.get_unchecked(n..) };
        self
    }

    /// Advance the view by n, advancing it in-place to (1..).
    unsafe fn step(&mut self) -> &mut Self {
        // SAFETY: safe as long as self is not empty
        unsafe { self.step_by(1) }
    }
}

const MIN_19DIGIT_INT: u64 = 100_0000_0000_0000_0000;

/// Parse up to 19 digits (the max that can be stored in a 64-bit integer).
pub fn try_parse_19digits(s: &mut AsciiStr<'_>, x: &mut u64) {
    while *x < MIN_19DIGIT_INT {
        if let Some(&c) = s.as_ref().first() {
            let digit = c - b'0';
            if digit < 10 {
                *x = (*x * 10) + digit as u64; // no overflows here
                // SAFETY: cannot be empty
                unsafe {
                    s.step();
                }
            } else {
                break;
            }
        } else {
            break;
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

pub fn main() {}
