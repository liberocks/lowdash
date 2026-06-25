use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_max(c: &mut Criterion) {
    let collection = support::float_vec(4_096);
    c.bench_function("max", |b| b.iter(|| ld::max(black_box(&collection))));
}
