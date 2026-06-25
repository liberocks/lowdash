use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_common_random_usize(c: &mut Criterion) {
    c.bench_function("common_random_usize/10000", |b| {
        b.iter(|| ld::common::random_usize(black_box(10_000)))
    });

    c.bench_function("common_random_usize/2", |b| {
        b.iter(|| ld::common::random_usize(black_box(2)))
    });
}
