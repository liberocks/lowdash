use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_fill(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("fill/int_vec/7", |b| {
        b.iter(|| ld::fill(black_box(&ints), black_box(7)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("fill/float_vec/3.14", |b| {
        b.iter(|| ld::fill(black_box(&floats), black_box(3.14)))
    });
}
