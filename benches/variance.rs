use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_variance(c: &mut Criterion) {
    let floats = support::float_vec(4_096);
    c.bench_function("variance/f64", |b| {
        b.iter(|| ld::variance(black_box(&floats)))
    });

    let small = support::float_vec(128);
    c.bench_function("variance/small", |b| {
        b.iter(|| ld::variance(black_box(&small)))
    });
}
