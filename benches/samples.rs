use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_samples(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("samples/int_vec/64", |b| {
        b.iter(|| ld::samples(black_box(&ints), black_box(64)))
    });

    c.bench_function("samples/int_vec/1", |b| {
        b.iter(|| ld::samples(black_box(&ints), black_box(1)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("samples/float_vec/64", |b| {
        b.iter(|| ld::samples(black_box(&floats), black_box(64)))
    });
}
