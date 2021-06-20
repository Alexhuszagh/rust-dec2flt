use core::fmt::{Debug, Display, LowerExp};
use core::ops::{Add, Div, Mul, Neg};

/// A helper trait to avoid duplicating basically all the conversion code for `f32` and `f64`.
///
/// See the parent module's doc comment for why this is necessary.
///
/// Should **never ever** be implemented for other types or be used outside the dec2flt module.
#[doc(hidden)]
pub trait RawFloat:
    Sized
    + Div<Output = Self>
    + Neg<Output = Self>
    + Mul<Output = Self>
    + Add<Output = Self>
    + LowerExp
    + PartialEq
    + PartialOrd
    + Default
    + Clone
    + Copy
    + Debug
    + Display
{
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NAN: Self;
    const NEG_NAN: Self;

    /// The number of bits in the significand, *excluding* the hidden bit.
    const MANTISSA_EXPLICIT_BITS: usize;

    // Round-to-even only happens for negative values of q
    // when q ≥ −4 in the 64-bit case and when q ≥ −17 in
    // the 32-bitcase.
    //
    // When q ≥ 0,we have that 5^q ≤ 2m+1. In the 64-bit case,we
    // have 5^q ≤ 2m+1 ≤ 2^54 or q ≤ 23. In the 32-bit case,we have
    // 5^q ≤ 2m+1 ≤ 2^25 or q ≤ 10.
    //
    // When q < 0, we have w ≥ (2m+1)×5^−q. We must have that w < 2^64
    // so (2m+1)×5^−q < 2^64. We have that 2m+1 > 2^53 (64-bit case)
    // or 2m+1 > 2^24 (32-bit case). Hence,we must have 2^53×5^−q < 2^64
    // (64-bit) and 2^24×5^−q < 2^64 (32-bit). Hence we have 5^−q < 2^11
    // or q ≥ −4 (64-bit case) and 5^−q < 2^40 or q ≥ −17 (32-bitcase).
    //
    // Thus we have that we only need to round ties to even when
    // we have that q ∈ [−4,23](in the 64-bit case) or q∈[−17,10]
    // (in the 32-bit case). In both cases,the power of five(5^|q|)
    // fits in a 64-bit word.
    const MIN_EXPONENT_ROUND_TO_EVEN: i32;
    const MAX_EXPONENT_ROUND_TO_EVEN: i32;

    // Minimum exponent that for a fast path case, or `-⌊(MANTISSA_EXPLICIT_BITS+1)/log2(5)⌋`
    const MIN_EXPONENT_FAST_PATH: i64;

    // Maximum exponent that for a fast path case, or `⌊(MANTISSA_EXPLICIT_BITS+1)/log2(5)⌋`
    const MAX_EXPONENT_FAST_PATH: i64;

    // Maximum exponent that can be represented for a disguised-fast path case.
    // This is `MAX_EXPONENT_FAST_PATH + ⌊(MANTISSA_EXPLICIT_BITS+1)/log2(10)⌋`
    const MAX_EXPONENT_DISGUISED_FAST_PATH: i64;

    // Minimum exponent value `(1 << (EXP_BITS - 1)) - 1`.
    const MINIMUM_EXPONENT: i32;

    // Largest exponent value `-((1 << EXP_BITS) - 1)`.
    const INFINITE_POWER: i32;

    // Index (in bits) of the sign.
    const SIGN_INDEX: usize;

    // Smallest decimal exponent for a non-zero value.
    const SMALLEST_POWER_OF_TEN: i32;

    // Largest decimal exponent for a non-infinite value.
    const LARGEST_POWER_OF_TEN: i32;

    // Maximum mantissa for the fast-path `(2^53 - 1)`.
    const MAX_MANTISSA_FAST_PATH: u64 = 2_u64 << Self::MANTISSA_EXPLICIT_BITS;

    fn from_u64(v: u64) -> Self;
    fn from_u64_bits(v: u64) -> Self;
    fn pow10_fast_path(exponent: usize) -> Self;
}

impl RawFloat for f32 {
    const INFINITY: Self = core::f32::INFINITY;
    const NEG_INFINITY: Self = core::f32::NEG_INFINITY;
    const NAN: Self = core::f32::NAN;
    const NEG_NAN: Self = -core::f32::NAN;

    const MANTISSA_EXPLICIT_BITS: usize = 23;
    const MIN_EXPONENT_ROUND_TO_EVEN: i32 = -17;
    const MAX_EXPONENT_ROUND_TO_EVEN: i32 = 10;
    const MIN_EXPONENT_FAST_PATH: i64 = -10; // assuming FLT_EVAL_METHOD = 0
    const MAX_EXPONENT_FAST_PATH: i64 = 10;
    const MAX_EXPONENT_DISGUISED_FAST_PATH: i64 = 17;
    const MINIMUM_EXPONENT: i32 = -127;
    const INFINITE_POWER: i32 = 0xFF;
    const SIGN_INDEX: usize = 31;
    const SMALLEST_POWER_OF_TEN: i32 = -65;
    const LARGEST_POWER_OF_TEN: i32 = 38;

    fn from_u64(v: u64) -> Self {
        v as _
    }

    fn from_u64_bits(v: u64) -> Self {
        f32::from_bits((v & 0xFFFFFFFF) as u32)
    }

    fn pow10_fast_path(exponent: usize) -> Self {
        #[allow(clippy::use_self)]
        const TABLE: [f32; 16] = [
            1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 0., 0., 0., 0., 0.,
        ];
        TABLE[exponent & 15]
    }
}

impl RawFloat for f64 {
    const INFINITY: Self = core::f64::INFINITY;
    const NEG_INFINITY: Self = core::f64::NEG_INFINITY;
    const NAN: Self = core::f64::NAN;
    const NEG_NAN: Self = -core::f64::NAN;

    const MANTISSA_EXPLICIT_BITS: usize = 52;
    const MIN_EXPONENT_ROUND_TO_EVEN: i32 = -4;
    const MAX_EXPONENT_ROUND_TO_EVEN: i32 = 23;
    const MIN_EXPONENT_FAST_PATH: i64 = -22; // assuming FLT_EVAL_METHOD = 0
    const MAX_EXPONENT_FAST_PATH: i64 = 22;
    const MAX_EXPONENT_DISGUISED_FAST_PATH: i64 = 37;
    const MINIMUM_EXPONENT: i32 = -1023;
    const INFINITE_POWER: i32 = 0x7FF;
    const SIGN_INDEX: usize = 63;
    const SMALLEST_POWER_OF_TEN: i32 = -342;
    const LARGEST_POWER_OF_TEN: i32 = 308;

    fn from_u64(v: u64) -> Self {
        v as _
    }

    fn from_u64_bits(v: u64) -> Self {
        f64::from_bits(v)
    }

    fn pow10_fast_path(exponent: usize) -> Self {
        const TABLE: [f64; 32] = [
            1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15,
            1e16, 1e17, 1e18, 1e19, 1e20, 1e21, 1e22, 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        ];
        TABLE[exponent & 31]
    }
}