use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_percentile(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("percentile", |b| {
        b.iter(|| {
            ld::percentile(black_box(&collection), black_box(95.0))
        })
    });
}
