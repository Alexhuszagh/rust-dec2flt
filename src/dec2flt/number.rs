use crate::dec2flt::float::RawFloat;

pub const INT_POW10: [u64; 16] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Number {
    pub exponent: i64,
    pub mantissa: u64,
    pub negative: bool,
    pub many_digits: bool,
}

impl Number {
    fn is_fast_path<F: RawFloat>(&self) -> bool {
        F::MIN_EXPONENT_FAST_PATH <= self.exponent
            && self.exponent <= F::MAX_EXPONENT_DISGUISED_FAST_PATH
            && self.mantissa <= F::MAX_MANTISSA_FAST_PATH
            && !self.many_digits
    }

    pub fn try_fast_path<F: RawFloat>(&self) -> Option<F> {
        if self.is_fast_path::<F>() {
            let mut value = if self.exponent <= F::MAX_EXPONENT_FAST_PATH {
                // normal fast path
                let value = F::from_u64(self.mantissa);
                if self.exponent < 0 {
                    value / F::pow10_fast_path((-self.exponent) as _)
                } else {
                    value * F::pow10_fast_path(self.exponent as _)
                }
            } else {
                // disguised fast path
                let shift = self.exponent - F::MAX_EXPONENT_FAST_PATH;
                let mantissa = self.mantissa.checked_mul(INT_POW10[shift as usize])?;
                if mantissa > F::MAX_MANTISSA_FAST_PATH {
                    return None;
                }
                F::from_u64(mantissa) * F::pow10_fast_path(F::MAX_EXPONENT_FAST_PATH as _)
            };
            if self.negative {
                value = -value;
            }
            Some(value)
        } else {
            None
        }
    }
}
