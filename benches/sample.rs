use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_sample(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("sample/int_vec", |b| {
        b.iter(|| ld::sample(black_box(&ints)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("sample/float_vec", |b| {
        b.iter(|| ld::sample(black_box(&floats)))
    });
}
