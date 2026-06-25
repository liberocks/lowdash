use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_pick_by_values(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    let values = vec![-1, 0, 1];
    c.bench_function("pick_by_values", |b| {
        b.iter(|| ld::pick_by_values(black_box(&map), black_box(&values)))
    });
}
