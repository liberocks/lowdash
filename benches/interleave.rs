use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_interleave(c: &mut Criterion) {
    let ragged = support::ragged_vecs();
    c.bench_function("interleave/ragged", |b| {
        b.iter(|| ld::interleave(black_box(&ragged)))
    });

    let regular = support::nested_vecs(16, 16);
    c.bench_function("interleave/regular/16x16", |b| {
        b.iter(|| ld::interleave(black_box(&regular)))
    });
}
