//! Slow, fallback algorithm for cases the Eisel-Lemire algorithm cannot round.

use crate::dec2flt::common::Fp;
use crate::dec2flt::decimal::{parse_decimal, Decimal};
use crate::dec2flt::float::RawFloat;

/// Parse the significant digits and biased, binary exponent of a float.
///
/// This is a fallback algorithm that uses a big-integer representation
/// of the float, and therefore is considerably slower than faster
/// approximations. However, it will always determine how to round
/// the significant digits to the nearest machine float, allowing
/// use to handle near half-way cases.
///
/// Near half-way cases are halfway between two consecutive machine floats.
/// For example, the float `16777217.0` has a bitwise representation of
/// `100000000000000000000000 1`. Rounding to a single-precision float,
/// the trailing `1` is truncated. Using round-nearest, tie-even, any
/// value above `16777217.0` must be rounded up to `16777218.0`, while
/// any value before or equal to `16777217.0` must be rounded down
/// to `16777216.0`. These near-halfway conversions therefore may require
/// a large number of digits to unambiguously determine how to round.
pub fn parse_long_mantissa<F: RawFloat>(s: &[u8]) -> Fp {
    const MAX_SHIFT: usize = 60;
    const NUM_POWERS: usize = 19;
    const POWERS: [u8; 19] = [
        0, 3, 6, 9, 13, 16, 19, 23, 26, 29, 33, 36, 39, 43, 46, 49, 53, 56, 59,
    ];

    let get_shift = |n| {
        if n < NUM_POWERS {
            POWERS[n] as usize
        } else {
            MAX_SHIFT
        }
    };

    let am_zero = Fp::zero_pow2(0);
    let am_inf = Fp::zero_pow2(F::INFINITE_POWER);

    let mut d = parse_decimal(s);

    if d.num_digits == 0 || d.decimal_point < -324 {
        return am_zero;
    } else if d.decimal_point >= 310 {
        return am_inf;
    }
    let mut exp2 = 0_i32;
    while d.decimal_point > 0 {
        let n = d.decimal_point as usize;
        let shift = get_shift(n);
        d.right_shift(shift);
        if d.decimal_point < -Decimal::DECIMAL_POINT_RANGE {
            return am_zero;
        }
        exp2 += shift as i32;
    }
    while d.decimal_point <= 0 {
        let shift = if d.decimal_point == 0 {
            match d.digits[0] {
                digit if digit >= 5 => break,
                0 | 1 => 2,
                _ => 1,
            }
        } else {
            get_shift((-d.decimal_point) as _)
        };
        d.left_shift(shift);
        if d.decimal_point > Decimal::DECIMAL_POINT_RANGE {
            return am_inf;
        }
        exp2 -= shift as i32;
    }
    exp2 -= 1;
    while (F::MINIMUM_EXPONENT + 1) > exp2 {
        let mut n = ((F::MINIMUM_EXPONENT + 1) - exp2) as usize;
        if n > MAX_SHIFT {
            n = MAX_SHIFT;
        }
        d.right_shift(n);
        exp2 += n as i32;
    }
    if (exp2 - F::MINIMUM_EXPONENT) >= F::INFINITE_POWER {
        return am_inf;
    }
    d.left_shift(F::MANTISSA_EXPLICIT_BITS + 1);
    let mut mantissa = d.round();
    if mantissa >= (1_u64 << (F::MANTISSA_EXPLICIT_BITS + 1)) {
        d.right_shift(1);
        exp2 += 1;
        mantissa = d.round();
        if (exp2 - F::MINIMUM_EXPONENT) >= F::INFINITE_POWER {
            return am_inf;
        }
    }
    let mut power2 = exp2 - F::MINIMUM_EXPONENT;
    if mantissa < (1_u64 << F::MANTISSA_EXPLICIT_BITS) {
        power2 -= 1;
    }
    mantissa &= (1_u64 << F::MANTISSA_EXPLICIT_BITS) - 1;
    Fp { mantissa, power2 }
}
