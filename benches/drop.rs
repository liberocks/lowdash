use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_drop(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("drop/int_vec/256", |b| {
        b.iter(|| ld::drop(black_box(&ints), black_box(256)))
    });

    c.bench_function("drop/int_vec/4096", |b| {
        b.iter(|| ld::drop(black_box(&ints), black_box(4096)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("drop/float_vec/256", |b| {
        b.iter(|| ld::drop(black_box(&floats), black_box(256)))
    });
}
