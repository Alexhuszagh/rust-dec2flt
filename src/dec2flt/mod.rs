//! Converting decimal strings into IEEE 754 binary floating point numbers.
//!
//! # Problem statement
//!
//! We are given a decimal string such as `12.34e56`. This string consists of integral (`12`),
//! fractional (`34`), and exponent (`56`) parts. All parts are optional and interpreted as zero
//! when missing.
//!
//! We seek the IEEE 754 floating point number that is closest to the exact value of the decimal
//! string. It is well-known that many decimal strings do not have terminating representations in
//! base two, so we round to 0.5 units in the last place (in other words, as well as possible).
//! Ties, decimal values exactly half-way between two consecutive floats, are resolved with the
//! half-to-even strategy, also known as banker's rounding.
//!
//! Needless to say, this is quite hard, both in terms of implementation complexity and in terms
//! of CPU cycles taken.
//!
//! # Implementation
//!
//! First, we ignore signs. Or rather, we remove it at the very beginning of the conversion
//! process and re-apply it at the very end. This is correct in all edge cases since IEEE
//! floats are symmetric around zero, negating one simply flips the first bit.
//!
//! Then we remove the decimal point by adjusting the exponent: Conceptually, `12.34e56` turns
//! into `1234e54`, which we describe with a positive integer `f = 1234` and an integer `e = 54`.
//! The `(f, e)` representation is used by almost all code past the parsing stage.
//!
//! We then try a long chain of progressively more general and expensive special cases using
//! machine-sized integers and small, fixed-sized floating point numbers (first `f32`/`f64`, then
//! a type with 64 bit significand, `Fp`). The extended-precision algorithm
//! uses the Eisel-Lemire algorithm, which uses a 128-bit (or 192-bit)
//! representation that can accurately and quickly compute the vast majority
//! of floats. When all these fail, we bite the bullet and resort to using
//! a large-decimal representation, shifting the digits into range, calculating
//! the upper significant bits and exactly round to the nearest representation.
//!
//! Another aspect that needs attention is the ``RawFloat`` trait by which almost all functions
//! are parametrized. One might think that it's enough to parse to `f64` and cast the result to
//! `f32`. Unfortunately this is not the world we live in, and this has nothing to do with using
//! base two or half-to-even rounding.
//!
//! Consider for example two types `d2` and `d4` representing a decimal type with two decimal
//! digits and four decimal digits each and take "0.01499" as input. Let's use half-up rounding.
//! Going directly to two decimal digits gives `0.01`, but if we round to four digits first,
//! we get `0.0150`, which is then rounded up to `0.02`. The same principle applies to other
//! operations as well, if you want 0.5 ULP accuracy you need to do *everything* in full precision
//! and round *exactly once, at the end*, by considering all truncated bits at once.
//!
//! Primarily, this module and its children implement the algorithms described in:
//! "Number Parsing at a Gigabyte per Second", available online:
//! <https://arxiv.org/abs/2101.11408>.
//!
//! # Other
//!
//! The conversion should *never* panic. There are assertions and explicit panics in the code,
//! but they should never be triggered and only serve as internal sanity checks. Any panics should
//! be considered a bug.
//!
//! There are unit tests but they are woefully inadequate at ensuring correctness, they only cover
//! a small percentage of possible errors. Far more extensive tests are located in the directory
//! `src/etc/test-float-parse` as a Python script.
//!
//! A note on integer overflow: Many parts of this file perform arithmetic with the decimal
//! exponent `e`. Primarily, we shift the decimal point around: Before the first decimal digit,
//! after the last decimal digit, and so on. This could overflow if done carelessly. We rely on
//! the parsing submodule to only hand out sufficiently small exponents, where "sufficient" means
//! "such that the exponent +/- the number of decimal digits fits into a 64 bit integer".
//! Larger exponents are accepted, but we don't do arithmetic with them, they are immediately
//! turned into {positive,negative} {zero,infinity}.

use core::fmt;

use self::binary::compute_float;
use self::float::RawFloat;
use self::parse::{parse_inf_nan, parse_number};
use self::simple::parse_long_mantissa;

mod binary;
mod common;
mod decimal;
mod float;
mod fpu;
mod number;
mod parse;
mod simple;
mod table;

/// An error which can be returned when parsing a float.
///
/// This error is used as the error type for the [`FromStr`] implementation
/// for [`f32`] and [`f64`].
///
/// # Example
///
/// ```
/// use std::str::FromStr;
///
/// if let Err(e) = f64::from_str("a.12") {
///     println!("Failed conversion to f64: {}", e);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseFloatError  {
    kind: FloatErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FloatErrorKind {
    Empty,
    Invalid,
}

impl ParseFloatError {
    #[doc(hidden)]
    pub fn __description(&self) -> &str {
        match self.kind {
            FloatErrorKind::Empty => "cannot parse float from empty string",
            FloatErrorKind::Invalid => "invalid float literal",
        }
    }
}

impl fmt::Display for ParseFloatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.__description().fmt(f)
    }
}

pub fn pfe_empty() -> ParseFloatError {
    ParseFloatError { kind: FloatErrorKind::Empty }
}

pub fn pfe_invalid() -> ParseFloatError {
    ParseFloatError { kind: FloatErrorKind::Invalid }
}

/// Converts a decimal string into a floating point number.
pub fn parse_float<F: RawFloat>(s: &str) -> Result<F, ParseFloatError> {
    let s = s.as_bytes();
    if s.is_empty() {
        return Err(pfe_empty());
    }

    let (num, rest) = match parse_number(s) {
        Some(r) => r,
        None => if let Some(value) = parse_inf_nan(s) {
            return Ok(value);
        } else {
            return Err(pfe_empty());
        },
    };
    if rest != s.len() {
        return Err(pfe_invalid());
    }
    if let Some(value) = num.try_fast_path::<F>() {
        return Ok(value);
    }

    let mut am = compute_float::<F>(num.exponent, num.mantissa);
    if num.many_digits && am != compute_float::<F>(num.exponent, num.mantissa + 1) {
        am.power2 = -1;
    }
    if am.power2 < 0 {
        am = parse_long_mantissa::<F>(s);
    }

    let mut word = am.mantissa;
    word |= (am.power2 as u64) << F::MANTISSA_EXPLICIT_BITS;
    if num.negative {
        word |= 1_u64 << F::SIGN_INDEX;
    }
    Ok(F::from_u64_bits(word))
}
