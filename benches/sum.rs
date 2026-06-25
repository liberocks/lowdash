use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_sum(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("sum", |b| b.iter(|| ld::sum(black_box(&collection))));
}
