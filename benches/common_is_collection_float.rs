use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_common_is_collection_float(c: &mut Criterion) {
    let collection = support::boxed_float_any(1_024);
    c.bench_function("common_is_collection_float", |b| {
        b.iter(|| {
            ld::common::is_collection_float(black_box(&collection))
        })
    });
}
