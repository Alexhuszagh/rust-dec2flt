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
