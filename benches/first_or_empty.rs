use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_first_or_empty(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("first_or_empty", |b| {
        b.iter(|| {
            ld::first_or_empty(black_box(&collection))
        })
    });
}
