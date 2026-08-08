use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_last_or_empty(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("last_or_empty/int_vec", |b| {
        b.iter(|| ld::last_or_empty(black_box(&ints)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("last_or_empty/float_vec", |b| {
        b.iter(|| ld::last_or_empty(black_box(&floats)))
    });
}
