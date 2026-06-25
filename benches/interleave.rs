use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_interleave(c: &mut Criterion) {
    let collection = support::ragged_vecs();
    c.bench_function("interleave", |b| {
        b.iter(|| ld::interleave(black_box(&collection)))
    });
}
