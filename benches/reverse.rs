use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_reverse(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("reverse/int_vec", |b| {
        b.iter(|| ld::reverse(black_box(&ints)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("reverse/float_vec", |b| {
        b.iter(|| ld::reverse(black_box(&floats)))
    });
}
