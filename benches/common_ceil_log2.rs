use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_common_ceil_log2(c: &mut Criterion) {
    c.bench_function("common_ceil_log2", |b| {
        b.iter(|| ld::common::ceil_log2(black_box(65_537)))
    });
}
