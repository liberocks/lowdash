use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_invert_grouped(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    c.bench_function("invert_grouped/medium", |b| {
        b.iter(|| ld::invert_grouped(black_box(&map)))
    });

    let small = support::numeric_map(64);
    c.bench_function("invert_grouped/small", |b| {
        b.iter(|| ld::invert_grouped(black_box(&small)))
    });
}
