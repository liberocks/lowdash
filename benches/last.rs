use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_last(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("last", |b| {
        b.iter(|| {
            ld::last(black_box(&collection))
        })
    });
}
