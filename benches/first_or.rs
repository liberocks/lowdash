use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_first_or(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("first_or/int_vec", |b| {
        b.iter(|| ld::first_or(black_box(&ints), black_box(-1)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("first_or/float_vec", |b| {
        b.iter(|| ld::first_or(black_box(&floats), black_box(-1.0)))
    });
}
