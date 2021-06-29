use core::marker::PhantomData;
use core::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsciiStr<'a> {
    ptr: *const u8,
    end: *const u8,
    _marker: PhantomData<&'a [u8]>,
}

impl<'a> AsciiStr<'a> {
    pub fn new(s: &'a [u8]) -> Self {
        Self {
            ptr: s.as_ptr(),
            end: unsafe { s.as_ptr().add(s.len()) },
            _marker: PhantomData::default(),
        }
    }

    /// Advance the view by n, advancing it in-place to (n..).
    pub fn step_by(&mut self, n: usize) -> &mut Self {
        unsafe { self.ptr = self.ptr.add(n) };
        self
    }

    /// Advance the view by n, advancing it in-place to (1..).
    pub fn step(&mut self) -> &mut Self {
        self.step_by(1)
    }

    /// Get if the slice contains no elements.
    pub fn is_empty(&self) -> bool {
        self.ptr == self.end
    }

    /// Get the first element in the slice, without bounds checking.
    pub fn first(&self) -> u8 {
        unsafe { *self.ptr }
    }

    /// Check if the first character in the slice is equal to c.
    pub fn first_is(&self, c: u8) -> bool {
        self.first() == c
    }

    /// Check if the first character in the slice is equal to c1 or c2.
    pub fn first_either(&self, c1: u8, c2: u8) -> bool {
        let c = self.first();
        c == c1 || c == c2
    }

    /// Bounds-checked test if the first character in the slice is equal to c.
    pub fn check_first(&self, c: u8) -> bool {
        !self.is_empty() && self.first() == c
    }

    /// Bounds-checked test if the first character in the slice is equal to c1 or c2.
    pub fn check_first_either(&self, c1: u8, c2: u8) -> bool {
        !self.is_empty() && (self.first() == c1 || self.first() == c2)
    }

    /// Bounds-checked test if the first character in the slice is a digit.
    pub fn check_first_digit(&self) -> bool {
        !self.is_empty() && self.first().is_ascii_digit()
    }

    /// Iteratively parse and consume digits from bytes.
    pub fn parse_digits(&mut self, mut func: impl FnMut(u8)) {
        while !self.is_empty() && self.first().is_ascii_digit() {
            func(self.first() - b'0');
            self.step();
        }
    }

    /// Check if the slice at least `n` length.
    pub fn check_len(&self, n: usize) -> bool {
        unsafe { n <= self.end.offset_from(self.ptr) as usize }
    }

    /// Try to read the next 8 bytes from the slice.
    pub fn try_read_u64(&self) -> Option<u64> {
        if self.check_len(8) {
            Some(self.read_u64())
        } else {
            None
        }
    }

    /// Read 8 bytes as a 64-bit integer in little-endian order.
    pub fn read_u64(&self) -> u64 {
        debug_assert!(self.check_len(8));
        let src = self.ptr as *const u64;
        u64::from_le(unsafe { ptr::read_unaligned(src) })
    }

    pub fn offset_from(&self, other: &Self) -> isize {
        isize::wrapping_sub(self.ptr as _, other.ptr as _) // assuming the same end
    }
}

// Most of these are inherently unsafe; we assume we know what we're calling and when.
pub trait ByteSlice: AsRef<[u8]> + AsMut<[u8]> {
    fn get_at(&self, i: usize) -> u8 {
        unsafe { *self.as_ref().get_unchecked(i) }
    }

    fn get_first(&self) -> u8 {
        debug_assert!(!self.as_ref().is_empty());
        self.get_at(0)
    }

    /// Check if the first character in the slice is equal to c.
    fn check_first(&self, c: u8) -> bool {
        !self.as_ref().is_empty() && self.get_first() == c
    }

    /// Check if the first character in the slice is equal to c1 or c2.
    fn check_first2(&self, c1: u8, c2: u8) -> bool {
        !self.as_ref().is_empty() && (self.get_first() == c1 || self.get_first() == c2)
    }

    /// Check if self starts with u with a case-insensitive comparison.
    fn eq_ignore_case(&self, u: &[u8]) -> bool {
        debug_assert!(self.as_ref().len() >= u.len());
        let d = (0..u.len()).fold(0, |d, i| d | self.get_at(i) ^ u.get_at(i));
        d == 0 || d == 32
    }

    /// Get the remaining slice after the first N elements.
    fn advance(&self, n: usize) -> &[u8] {
        &self.as_ref()[n..]
    }

    /// Get the slice after skipping all leading characters equal c.
    fn skip_chars(&self, c: u8) -> &[u8] {
        let mut s = self.as_ref();
        while s.check_first(c) {
            s = s.advance(1);
        }
        s
    }

    /// Get the slice after skipping all leading characters equal c1 or c2.
    fn skip_chars2(&self, c1: u8, c2: u8) -> &[u8] {
        let mut s = self.as_ref();
        while !s.is_empty() && (s.get_first() == c1 || s.get_first() == c2) {
            s = s.advance(1);
        }
        s
    }

    /// Read 8 bytes as a 64-bit integer in little-endian order.
    fn read_u64(&self) -> u64 {
        debug_assert!(self.as_ref().len() >= 8);
        let src = self.as_ref().as_ptr() as *const u64;
        u64::from_le(unsafe { ptr::read_unaligned(src) })
    }

    /// Write a 64-bit integer as 8 bytes in little-endian order.
    fn write_u64(&mut self, value: u64) {
        debug_assert!(self.as_ref().len() >= 8);
        let dst = self.as_mut().as_mut_ptr() as *mut u64;
        unsafe { ptr::write_unaligned(dst, u64::to_le(value)) };
    }
}

impl ByteSlice for [u8] {}

/// Determine if 8 bytes, loaded in LE-order, are all decimal digits.
pub fn is_8digits(v: u64) -> bool {
    let a = v.wrapping_add(0x4646_4646_4646_4646);
    let b = v.wrapping_sub(0x3030_3030_3030_3030);
    (a | b) & 0x8080_8080_8080_8080 == 0
}

/// Iteratively parse and consume digits from bytes.
pub fn parse_digits(s: &mut &[u8], mut f: impl FnMut(u8)) {
    while !s.is_empty() {
        let c = s.get_first().wrapping_sub(b'0');
        if c < 10 {
            f(c);
            *s = s.advance(1);
        } else {
            break;
        }
    }
}

/// A custom 64-bit floating point type, representing `mantissa * 2^e`.
/// power2 is biased, so it be directly shifted into the exponent bits.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Fp {
    pub mantissa: u64,
    pub power2: i32,
}

impl Fp {
    pub const fn zero_pow2(power2: i32) -> Self {
        Self {
            mantissa: 0,
            power2,
        }
    }
}
