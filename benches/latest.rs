use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_latest(c: &mut Criterion) {
    let time_increasing = support::time_vec(4_096);
    c.bench_function("latest/time_vec/increasing", |b| {
        b.iter(|| ld::latest(black_box(&time_increasing)))
    });

    let time_descending = support::time_vec_descending(4_096);
    c.bench_function("latest/time_vec/descending", |b| {
        b.iter(|| ld::latest(black_box(&time_descending)))
    });

    let time_equal = support::time_vec_equal(4_096);
    c.bench_function("latest/time_vec/equal", |b| {
        b.iter(|| ld::latest(black_box(&time_equal)))
    });

    let time_shuffled = support::time_vec_shuffled(4_096);
    c.bench_function("latest/time_vec/shuffled", |b| {
        b.iter(|| ld::latest(black_box(&time_shuffled)))
    });
}
