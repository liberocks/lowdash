use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_samples(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("samples", |b| {
        b.iter(|| ld::samples(black_box(&collection), black_box(64)))
    });
}
