use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_flatten(c: &mut Criterion) {
    let collection = support::nested_vecs(128, 8);
    c.bench_function("flatten", |b| {
        b.iter(|| {
            ld::flatten(black_box(&collection))
        })
    });
}
