use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_common_random_u64(c: &mut Criterion) {
    c.bench_function("common_random_u64", |b| {
        b.iter(|| {
            ld::common::random_u64()
        })
    });
}
