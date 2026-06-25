use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_from_pairs(c: &mut Criterion) {
    let entries = support::entry_vec(2_048);
    c.bench_function("from_pairs", |b| {
        b.iter(|| {
            ld::from_pairs(black_box(&entries))
        })
    });
}
