//! Implementation of the Eisel-Lemire algorithm.

use crate::dec2flt::common::Fp;
use crate::dec2flt::float::RawFloat;
use crate::dec2flt::table::{LARGEST_POWER_OF_FIVE, POWER_OF_FIVE_128, SMALLEST_POWER_OF_FIVE};

/// Compute a float using an extended-precision representation.
///
/// Fast conversion of a the significant digits and decimal exponent
/// a float to a extended representation with a binary float. This
/// algorithm will accurately parse the vast majority of cases,
/// and uses a 128-bit representation (with a fallback 192-bit
/// representation).
///
/// This algorithm scales the exponent by the decimal exponent
/// using pre-computed powers-of-5, and calculates if the
/// representation can be unambiguously rounded to the nearest
/// machine float. Near-halfway cases are not handled here,
/// and are represented by a negative, biased binary exponent.
pub fn compute_float<F: RawFloat>(q: i64, mut w: u64) -> Fp {
    let am_zero = Fp::zero_pow2(0);
    let am_inf = Fp::zero_pow2(F::INFINITE_POWER);
    let am_error = Fp::zero_pow2(-1);

    if w == 0 || q < F::SMALLEST_POWER_OF_TEN as i64 {
        return am_zero;
    } else if q > F::LARGEST_POWER_OF_TEN as i64 {
        return am_inf;
    }
    let lz = w.leading_zeros();
    w <<= lz;
    let (lo, hi) = compute_product_approx(q, w, F::MANTISSA_EXPLICIT_BITS + 3);
    if lo == 0xFFFF_FFFF_FFFF_FFFF {
        let inside_safe_exponent = (q >= -27) && (q <= 55);
        if !inside_safe_exponent {
            return am_error;
        }
    }
    let upperbit = (hi >> 63) as i32;
    let mut mantissa = hi >> (upperbit + 64 - F::MANTISSA_EXPLICIT_BITS as i32 - 3);
    let mut power2 = power(q as i32) + upperbit - lz as i32 - F::MINIMUM_EXPONENT;
    if power2 <= 0 {
        if -power2 + 1 >= 64 {
            return am_zero;
        }
        mantissa >>= -power2 + 1;
        mantissa += mantissa & 1;
        mantissa >>= 1;
        power2 = (mantissa >= (1_u64 << F::MANTISSA_EXPLICIT_BITS)) as i32;
        return Fp { mantissa, power2 };
    }
    if lo <= 1
        && q >= F::MIN_EXPONENT_ROUND_TO_EVEN as i64
        && q <= F::MAX_EXPONENT_ROUND_TO_EVEN as i64
        && mantissa & 3 == 1
        && (mantissa << (upperbit + 64 - F::MANTISSA_EXPLICIT_BITS as i32 - 3)) == hi
    {
        mantissa &= !1_u64;
    }
    mantissa += mantissa & 1;
    mantissa >>= 1;
    if mantissa >= (2_u64 << F::MANTISSA_EXPLICIT_BITS) {
        mantissa = 1_u64 << F::MANTISSA_EXPLICIT_BITS;
        power2 += 1;
    }
    mantissa &= !(1_u64 << F::MANTISSA_EXPLICIT_BITS);
    if power2 >= F::INFINITE_POWER {
        return am_inf;
    }
    Fp { mantissa, power2 }
}

fn power(q: i32) -> i32 {
    (q.wrapping_mul(152_170 + 65536) >> 16) + 63
}

fn full_multiplication(a: u64, b: u64) -> (u64, u64) {
    let r = (a as u128) * (b as u128);
    (r as u64, (r >> 64) as u64)
}

// This will compute or rather approximate w * 5**q and return a pair of 64-bit words
// approximating the result, with the "high" part corresponding to the most significant
// bits and the low part corresponding to the least significant bits.
fn compute_product_approx(q: i64, w: u64, precision: usize) -> (u64, u64) {
    debug_assert!(q >= SMALLEST_POWER_OF_FIVE as i64);
    debug_assert!(q <= LARGEST_POWER_OF_FIVE as i64);
    debug_assert!(precision <= 64);

    let mask = if precision < 64 {
        0xFFFF_FFFF_FFFF_FFFF_u64 >> precision
    } else {
        0xFFFF_FFFF_FFFF_FFFF_u64
    };
    let index = (q - SMALLEST_POWER_OF_FIVE as i64) as usize;
    let (lo5, hi5) = unsafe { *POWER_OF_FIVE_128.get_unchecked(index) };
    let (mut first_lo, mut first_hi) = full_multiplication(w, lo5);
    if first_hi & mask == mask {
        let (_, second_hi) = full_multiplication(w, hi5);
        first_lo = first_lo.wrapping_add(second_hi);
        if second_hi > first_lo {
            first_hi += 1;
        }
    }
    (first_lo, first_hi)
}