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

    pub fn step_by(&mut self, n: usize) -> &mut Self {
        unsafe { self.ptr = self.ptr.add(n) };
        self
    }

    pub fn step(&mut self) -> &mut Self {
        self.step_by(1)
    }

    pub fn is_empty(&self) -> bool {
        self.ptr == self.end
    }

    pub fn first(&self) -> u8 {
        unsafe { *self.ptr }
    }

    pub fn first_is(&self, c: u8) -> bool {
        self.first() == c
    }

    pub fn first_either(&self, c1: u8, c2: u8) -> bool {
        let c = self.first();
        c == c1 || c == c2
    }

    pub fn check_first(&self, c: u8) -> bool {
        !self.is_empty() && self.first() == c
    }

    pub fn check_first_either(&self, c1: u8, c2: u8) -> bool {
        !self.is_empty() && (self.first() == c1 || self.first() == c2)
    }

    pub fn check_first_digit(&self) -> bool {
        !self.is_empty() && self.first().is_ascii_digit()
    }

    pub fn parse_digits(&mut self, mut func: impl FnMut(u8)) {
        while !self.is_empty() && self.first().is_ascii_digit() {
            func(self.first() - b'0');
            self.step();
        }
    }

    pub fn check_len(&self, n: usize) -> bool {
        unsafe { self.ptr.add(n) <= self.end }
    }

    pub fn try_read_u64(&self) -> Option<u64> {
        if self.check_len(8) {
            Some(self.read_u64())
        } else {
            None
        }
    }

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

    fn check_first(&self, c: u8) -> bool {
        !self.as_ref().is_empty() && self.get_first() == c
    }

    fn check_first2(&self, c1: u8, c2: u8) -> bool {
        !self.as_ref().is_empty() && (self.get_first() == c1 || self.get_first() == c2)
    }

    fn eq_ignore_case(&self, u: &[u8]) -> bool {
        debug_assert!(self.as_ref().len() >= u.len());
        let d = (0..u.len()).fold(0, |d, i| d | self.get_at(i) ^ u.get_at(i));
        d == 0 || d == 32
    }

    fn advance(&self, n: usize) -> &[u8] {
        &self.as_ref()[n..]
    }

    fn skip_chars(&self, c: u8) -> &[u8] {
        let mut s = self.as_ref();
        while s.check_first(c) {
            s = s.advance(1);
        }
        s
    }

    fn skip_chars2(&self, c1: u8, c2: u8) -> &[u8] {
        let mut s = self.as_ref();
        while !s.is_empty() && (s.get_first() == c1 || s.get_first() == c2) {
            s = s.advance(1);
        }
        s
    }

    fn read_u64(&self) -> u64 {
        debug_assert!(self.as_ref().len() >= 8);
        let src = self.as_ref().as_ptr() as *const u64;
        u64::from_le(unsafe { ptr::read_unaligned(src) })
    }

    fn write_u64(&mut self, value: u64) {
        debug_assert!(self.as_ref().len() >= 8);
        let dst = self.as_mut().as_mut_ptr() as *mut u64;
        unsafe { ptr::write_unaligned(dst, u64::to_le(value)) };
    }
}

impl ByteSlice for [u8] {}

pub fn is_8digits(v: u64) -> bool {
    let a = v.wrapping_add(0x4646_4646_4646_4646);
    let b = v.wrapping_sub(0x3030_3030_3030_3030);
    (a | b) & 0x8080_8080_8080_8080 == 0
}

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct AdjustedMantissa {
    pub mantissa: u64,
    pub power2: i32,
}

impl AdjustedMantissa {
    pub const fn zero_pow2(power2: i32) -> Self {
        Self {
            mantissa: 0,
            power2,
        }
    }
}
