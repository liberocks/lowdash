use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_entries(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    c.bench_function("entries", |b| b.iter(|| ld::entries(black_box(&map))));
}
