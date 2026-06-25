use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_to_pairs(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    c.bench_function("to_pairs", |b| {
        b.iter(|| {
            ld::to_pairs(black_box(&map))
        })
    });
}
