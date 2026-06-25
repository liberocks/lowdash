use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_median(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("median", |b| b.iter(|| ld::median(black_box(&collection))));
}
