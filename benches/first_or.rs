use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_first_or(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("first_or", |b| {
        b.iter(|| ld::first_or(black_box(&collection), black_box(-1)))
    });
}
