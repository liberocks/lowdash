use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_chunk(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("chunk/int_vec/32", |b| {
        b.iter(|| ld::chunk(black_box(&ints), black_box(32)))
    });

    c.bench_function("chunk/int_vec/128", |b| {
        b.iter(|| ld::chunk(black_box(&ints), black_box(128)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("chunk/float_vec/32", |b| {
        b.iter(|| ld::chunk(black_box(&floats), black_box(32)))
    });
}
