use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_last(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("last/int_vec", |b| b.iter(|| ld::last(black_box(&ints))));

    let floats = support::float_vec(4_096);
    c.bench_function("last/float_vec", |b| {
        b.iter(|| ld::last(black_box(&floats)))
    });
}
