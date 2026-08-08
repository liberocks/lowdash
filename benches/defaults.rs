use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_defaults(c: &mut Criterion) {
    let small = support::numeric_maps(8, 256);
    c.bench_function("defaults/small", |b| {
        b.iter(|| ld::defaults(black_box(&small)))
    });

    let large = support::numeric_maps(32, 1_024);
    c.bench_function("defaults/large", |b| {
        b.iter(|| ld::defaults(black_box(&large)))
    });
}
