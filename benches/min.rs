use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_min(c: &mut Criterion) {
    let collection = support::float_vec(4_096);
    c.bench_function("min", |b| {
        b.iter(|| {
            ld::min(black_box(&collection))
        })
    });
}
