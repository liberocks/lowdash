use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_percentile(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("percentile/int_vec/95", |b| {
        b.iter(|| ld::percentile(black_box(&ints), black_box(95.0)))
    });

    c.bench_function("percentile/int_vec/50", |b| {
        b.iter(|| ld::percentile(black_box(&ints), black_box(50.0)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("percentile/float_vec/95", |b| {
        b.iter(|| ld::percentile(black_box(&floats), black_box(95.0)))
    });
}
