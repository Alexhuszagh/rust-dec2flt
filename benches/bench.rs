//! Benchmark sample floats meant to invoke certain code paths.

use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dec2flt::parse;

macro_rules! bench_generator {
    ($group:ident, $name:literal, $str:ident, $t:ty) => {
        $group.bench_function($name, |bench| {
            bench.iter(|| {
                black_box(parse::<$t>($str).unwrap());
            })
        });
    };
}

// FLOATS

// NOTE: Rust currently doesn't handle large, denormal floats
// with more than 25 significant digits. Use the 25 significant
// digits for both large and denormal.

// Example fast-path value.
const FAST_PATH: &str = "1.2345e22";
// Example disguised fast-path value.
const DISGUISED_FAST_PATH: &str = "1.2345e30";
// Example moderate path value: clearly not halfway `1 << 53`.
const MODERATE_PATH: &str = "9007199254740992.0";
// Example exactly-halfway value `(1<<53) + 1`.
const HALFWAY: &str = "9007199254740993.0";
// Example large, near-halfway value.
const LARGE: &str = "8.988465674311580536566680e307";
// Example denormal, near-halfway value.
const DENORMAL: &str = "8.442911973260991817129021e-309";

fn dec2flt(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("dec2flt");
    group.measurement_time(Duration::from_secs(5));
    bench_generator!(group, "fast", FAST_PATH, f64);
    bench_generator!(group, "disguised", DISGUISED_FAST_PATH, f64);
    bench_generator!(group, "moderate", MODERATE_PATH, f64);
    bench_generator!(group, "halfway", HALFWAY, f64);
    bench_generator!(group, "large", LARGE, f64);
    bench_generator!(group, "denormal", DENORMAL, f64);
}

// MAIN

criterion_group!(dec2flt_benches, dec2flt);
criterion_main!(dec2flt_benches);
