use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_last_or(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("last_or/int_vec", |b| {
        b.iter(|| ld::last_or(black_box(&ints), black_box(-1)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("last_or/float_vec", |b| {
        b.iter(|| ld::last_or(black_box(&floats), black_box(-1.0)))
    });
}
