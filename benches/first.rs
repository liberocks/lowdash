use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_first(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("first", |b| b.iter(|| ld::first(black_box(&collection))));
}
