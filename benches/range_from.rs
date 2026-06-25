use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_range_from(c: &mut Criterion) {
    c.bench_function("range_from/500-10000", |b| {
        b.iter(|| ld::range_from(black_box(500_i32), black_box(10_000)))
    });

    c.bench_function("range_from/0-100", |b| {
        b.iter(|| ld::range_from(black_box(0_i32), black_box(100)))
    });
}
