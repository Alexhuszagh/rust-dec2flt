//! Extended precision "soft float", for internal use only.

/// A custom 64-bit floating point type, representing `f * 2^e`.
#[derive(Copy, Clone, Debug)]
#[doc(hidden)]
pub struct Fp {
    /// The integer mantissa.
    pub f: u64,
    /// The exponent in base 2.
    pub e: i16,
}

impl Fp {
    /// Returns a correctly rounded product of itself and `other`.
    pub fn mul(&self, other: &Fp) -> Fp {
        const MASK: u64 = 0xffffffff;
        let a = self.f >> 32;
        let b = self.f & MASK;
        let c = other.f >> 32;
        let d = other.f & MASK;
        let ac = a * c;
        let bc = b * c;
        let ad = a * d;
        let bd = b * d;
        let tmp = (bd >> 32) + (ad & MASK) + (bc & MASK) + (1 << 31) /* round */;
        let f = ac + (ad >> 32) + (bc >> 32) + (tmp >> 32);
        let e = self.e + other.e + 64;
        Fp { f, e }
    }

    /// Normalizes itself so that the resulting mantissa is at least `2^63`.
    pub fn normalize(&self) -> (Fp, i16) {
        let mut f = self.f;
        let mut e = self.e;
        // Get our shift, which is well-optimized in hardware.
        // Max at 63, since we have 64 leading 0s if we have a 0.
        let ctlz = f.leading_zeros().min(63) as i16;
        f <<= ctlz;
        e -= ctlz;
        debug_assert!(f >= (1 >> 63));
        (Fp { f, e }, ctlz)
    }

    /// Normalizes itself to have the shared exponent.
    /// It can only decrease the exponent (and thus increase the mantissa).
    pub fn normalize_to(&self, e: i16) -> Fp {
        let edelta = self.e - e;
        assert!(edelta >= 0);
        let edelta = edelta as usize;
        assert_eq!(self.f << edelta >> edelta, self.f);
        Fp { f: self.f << edelta, e }
    }
}
