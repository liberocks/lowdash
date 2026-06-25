use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_count(c: &mut Criterion) {
    let duplicates = support::duplicate_int_vec(4_096);
    c.bench_function("count/duplicate_int_vec/7", |b| {
        b.iter(|| ld::count(black_box(&duplicates), black_box(7)))
    });

    let ints = support::int_vec(4_096);
    c.bench_function("count/int_vec/7", |b| {
        b.iter(|| ld::count(black_box(&ints), black_box(7)))
    });
}
