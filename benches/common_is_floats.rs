use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_common_is_floats(c: &mut Criterion) {
    c.bench_function("common_is_floats/f64", |b| {
        b.iter(|| black_box(ld::common::is_floats::<f64>()))
    });

    c.bench_function("common_is_floats/i32", |b| {
        b.iter(|| black_box(ld::common::is_floats::<i32>()))
    });
}
