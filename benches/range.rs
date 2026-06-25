use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_range(c: &mut Criterion) {
    c.bench_function("range", |b| {
        b.iter(|| {
            ld::range(black_box(10_000))
        })
    });
}
