use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_sample(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("sample", |b| {
        b.iter(|| {
            ld::sample(black_box(&collection))
        })
    });
}
