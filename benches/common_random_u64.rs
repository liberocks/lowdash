use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_common_random_u64(c: &mut Criterion) {
    c.bench_function("common_random_u64", |b| {
        b.iter(|| black_box(ld::common::random_u64()))
    });
}
