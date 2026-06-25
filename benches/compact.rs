use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_compact(c: &mut Criterion) {
    let defaulty = support::defaulty_int_vec(4_096);
    c.bench_function("compact/defaulty_int_vec", |b| {
        b.iter(|| ld::compact(black_box(&defaulty)))
    });

    let ints = support::int_vec(4_096);
    c.bench_function("compact/int_vec", |b| {
        b.iter(|| ld::compact(black_box(&ints)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("compact/float_vec", |b| {
        b.iter(|| ld::compact(black_box(&floats)))
    });
}
