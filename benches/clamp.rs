use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_clamp(c: &mut Criterion) {
    c.bench_function("clamp/i32/within", |b| {
        b.iter(|| ld::clamp(black_box(123_i32), black_box(-10_i32), black_box(90_i32)))
    });

    c.bench_function("clamp/i32/below", |b| {
        b.iter(|| ld::clamp(black_box(-50_i32), black_box(-10_i32), black_box(90_i32)))
    });

    c.bench_function("clamp/i32/above", |b| {
        b.iter(|| ld::clamp(black_box(999_i32), black_box(-10_i32), black_box(90_i32)))
    });

    c.bench_function("clamp/f64/within", |b| {
        b.iter(|| ld::clamp(black_box(3.14_f64), black_box(0.0_f64), black_box(10.0_f64)))
    });
}
