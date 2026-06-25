use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_range_from(c: &mut Criterion) {
    c.bench_function("range_from", |b| {
        b.iter(|| ld::range_from(black_box(500_i32), black_box(10_000)))
    });
}
