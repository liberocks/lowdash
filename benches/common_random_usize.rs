use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_common_random_usize(c: &mut Criterion) {
    c.bench_function("common_random_usize", |b| {
        b.iter(|| {
            ld::common::random_usize(black_box(10_000))
        })
    });
}
