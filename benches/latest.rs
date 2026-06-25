use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_latest(c: &mut Criterion) {
    let collection = support::time_vec(4_096);
    c.bench_function("latest", |b| b.iter(|| ld::latest(black_box(&collection))));
}
