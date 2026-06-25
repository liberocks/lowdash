use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_pick_by_values(c: &mut Criterion) {
    let values = vec![-1, 0, 1];
    let map = support::numeric_map(2_048);
    c.bench_function("pick_by_values/medium", |b| {
        b.iter(|| ld::pick_by_values(black_box(&map), black_box(&values)))
    });

    let small = support::numeric_map(64);
    c.bench_function("pick_by_values/small", |b| {
        b.iter(|| ld::pick_by_values(black_box(&small), black_box(&values)))
    });
}
