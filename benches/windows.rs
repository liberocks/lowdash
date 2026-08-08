use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_windows(c: &mut Criterion) {
    let numbers = support::int_vec(4_096);
    c.bench_function("windows/int_vec/8", |b| {
        b.iter(|| ld::windows(black_box(&numbers), black_box(8)))
    });

    c.bench_function("windows/int_vec/64", |b| {
        b.iter(|| ld::windows(black_box(&numbers), black_box(64)))
    });
}
