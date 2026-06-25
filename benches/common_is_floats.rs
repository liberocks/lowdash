use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_common_is_floats(c: &mut Criterion) {
    c.bench_function("common_is_floats", |b| {
        b.iter(|| {
            ld::common::is_floats::<f64>()
        })
    });
}
