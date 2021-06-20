#![feature(llvm_asm)]

use fast_float::parse;

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

pub fn main() {
    black_box(parse::<f64, _>("1.2345").unwrap());
}
