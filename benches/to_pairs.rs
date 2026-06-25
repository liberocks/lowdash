use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_to_pairs(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    c.bench_function("to_pairs/medium", |b| b.iter(|| ld::to_pairs(black_box(&map))));

    let small = support::numeric_map(64);
    c.bench_function("to_pairs/small", |b| b.iter(|| ld::to_pairs(black_box(&small))));
}
