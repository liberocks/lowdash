use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_is_sorted(c: &mut Criterion) {
    let increasing: Vec<i32> = (0..4_096).collect();
    c.bench_function("is_sorted/increasing", |b| {
        b.iter(|| ld::is_sorted(black_box(&increasing)))
    });

    let descending = support::int_vec_descending(4_096);
    c.bench_function("is_sorted/descending", |b| {
        b.iter(|| ld::is_sorted(black_box(&descending)))
    });

    let shuffled = support::int_vec_shuffled(4_096);
    c.bench_function("is_sorted/shuffled", |b| {
        b.iter(|| ld::is_sorted(black_box(&shuffled)))
    });
}
