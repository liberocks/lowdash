use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_first_or_empty(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("first_or_empty/int_vec", |b| {
        b.iter(|| ld::first_or_empty(black_box(&ints)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("first_or_empty/float_vec", |b| {
        b.iter(|| ld::first_or_empty(black_box(&floats)))
    });
}
