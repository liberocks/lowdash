use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_fill(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("fill", |b| {
        b.iter(|| {
            ld::fill(black_box(&collection), black_box(7))
        })
    });
}
