use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_mean(c: &mut Criterion) {
    let floats = support::float_vec(4_096);
    c.bench_function("mean/float_vec/large", |b| b.iter(|| ld::mean(black_box(&floats))));

    let small = support::float_vec(128);
    c.bench_function("mean/float_vec/small", |b| b.iter(|| ld::mean(black_box(&small))));

    let ints = support::int_vec(4_096);
    c.bench_function("mean/int_vec", |b| b.iter(|| ld::mean(black_box(&ints))));
}
