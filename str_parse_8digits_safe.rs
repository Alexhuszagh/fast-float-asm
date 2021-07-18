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

    fn try_step_by(&mut self, n: usize) -> Option<&mut Self> {
        if self.check_len(n) {
            Some(unsafe { self.step_by(n) })
        } else {
            None
        }
    }

    /// Iteratively parse and consume digits from bytes.
    fn parse_digits(&mut self, mut func: impl FnMut(u8)) {
        while let Some(&c) = self.as_ref().first() {
            let c = c.wrapping_sub(b'0');
            if c < 10 {
                func(c);
                // SAFETY: self cannot be empty
                unsafe {
                    self.step();
                }
            } else {
                break;
            }
        }
    }
}

/// Determine if 8 bytes are all decimal digits.
/// This does not care about the order in which the bytes were loaded.
fn is_8digits(v: u64) -> bool {
    let a = v.wrapping_add(0x4646_4646_4646_4646);
    let b = v.wrapping_sub(0x3030_3030_3030_3030);
    (a | b) & 0x8080_8080_8080_8080 == 0
}

/// Parse 8 digits, loaded as bytes in little-endian order.
///
/// This uses the trick where every digit is in [0x030, 0x39],
/// and therefore can be parsed in 3 multiplications, much
/// faster than the normal 8.
///
/// This is based off the algorithm described in "Fast numeric string to
/// int", available here: <https://johnnylee-sde.github.io/Fast-numeric-string-to-int/>.
fn parse_8digits(mut v: u64) -> u64 {
    const MASK: u64 = 0x0000_00FF_0000_00FF;
    const MUL1: u64 = 0x000F_4240_0000_0064;
    const MUL2: u64 = 0x0000_2710_0000_0001;
    v -= 0x3030_3030_3030_3030;
    v = (v * 10) + (v >> 8); // will not overflow, fits in 63 bits
    let v1 = (v & MASK).wrapping_mul(MUL1);
    let v2 = ((v >> 16) & MASK).wrapping_mul(MUL2);
    ((v1.wrapping_add(v2) >> 32) as u32) as u64
}

/// Try to parse 8 digits at a time, using an optimized algorithm.
pub fn try_parse_8digits<'a>(mut s: &'a mut AsciiStr<'a>, x: &mut u64) -> &'a mut AsciiStr<'a> {
    // may cause overflows, to be handled later
    if let Some(v) = s.read_u64() {
        if is_8digits(v) {
            *x = x.wrapping_mul(1_0000_0000).wrapping_add(parse_8digits(v));
            s = s.try_step_by(8).unwrap();
        }
    }
    s
}

impl<'a> AsRef<[u8]> for AsciiStr<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.slc
    }
}

impl<'a> ByteSlice for AsciiStr<'a> {}

pub fn main() {}
