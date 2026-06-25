use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_count_values(c: &mut Criterion) {
    let duplicates = support::duplicate_int_vec(4_096);
    c.bench_function("count_values/duplicate_int_vec", |b| {
        b.iter(|| ld::count_values(black_box(&duplicates)))
    });

    let ints = support::int_vec(4_096);
    c.bench_function("count_values/int_vec", |b| {
        b.iter(|| ld::count_values(black_box(&ints)))
    });
}
