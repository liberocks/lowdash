use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_flatten(c: &mut Criterion) {
    let nested = support::nested_vecs(128, 32);
    c.bench_function("flatten/nested/128x32", |b| {
        b.iter(|| ld::flatten(black_box(&nested)))
    });

    let small = support::nested_vecs(32, 8);
    c.bench_function("flatten/nested/32x8", |b| {
        b.iter(|| ld::flatten(black_box(&small)))
    });

    let ragged = support::ragged_vecs();
    c.bench_function("flatten/ragged", |b| {
        b.iter(|| ld::flatten(black_box(&ragged)))
    });
}
