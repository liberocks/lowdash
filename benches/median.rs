use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_median(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("median/int_vec", |b| {
        b.iter(|| ld::median(black_box(&ints)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("median/float_vec", |b| {
        b.iter(|| ld::median(black_box(&floats)))
    });
}
