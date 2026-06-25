use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_find_uniques(c: &mut Criterion) {
    let duplicates = support::duplicate_int_vec(4_096);
    c.bench_function("find_uniques/duplicate_int_vec", |b| {
        b.iter(|| ld::find_uniques(black_box(&duplicates)))
    });

    let ints = support::int_vec(4_096);
    c.bench_function("find_uniques/int_vec", |b| {
        b.iter(|| ld::find_uniques(black_box(&ints)))
    });
}
