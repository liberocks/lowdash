use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_common_is_collection_float(c: &mut Criterion) {
    let floats = support::boxed_float_any(1_024);
    c.bench_function("common_is_collection_float/1k", |b| {
        b.iter(|| ld::common::is_collection_float(black_box(&floats)))
    });

    let large = support::boxed_float_any(4_096);
    c.bench_function("common_is_collection_float/4k", |b| {
        b.iter(|| ld::common::is_collection_float(black_box(&large)))
    });
}
