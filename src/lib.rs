//! Numeric traits and functions for the built-in numeric types.

#![feature(allow_internal_unstable)]
#![feature(core_intrinsics)]
#![feature(iter_zip)]
#![feature(llvm_asm)]
#![no_std]

// All these modules are technically private and only exposed for coretests:
pub mod bignum;
pub mod dec2flt;
pub mod diy_float;

pub use self::dec2flt::dec2flt as parse;

/// Sample function to prevent optimization for binaries.
#[inline]
pub fn black_box<T>(mut dummy: T) -> T {
    // SAFETY: the inline assembly is a no-op.
    unsafe {
        // FIXME: Cannot use `asm!` because it doesn't support MIPS and other architectures.
        llvm_asm!("" : : "r"(&mut dummy) : "memory" : "volatile");
    }
    dummy
}
