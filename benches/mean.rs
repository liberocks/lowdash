use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_mean(c: &mut Criterion) {
    let collection = support::float_vec(128);
    c.bench_function("mean", |b| b.iter(|| ld::mean(black_box(&collection))));
}
