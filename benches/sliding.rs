use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_sliding(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("sliding/int_vec/8/1", |b| {
        b.iter(|| ld::sliding(black_box(&ints), black_box(8), black_box(1)))
    });

    c.bench_function("sliding/int_vec/64/64", |b| {
        b.iter(|| ld::sliding(black_box(&ints), black_box(64), black_box(64)))
    });
}
