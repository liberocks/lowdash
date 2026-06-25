use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_find_duplicates(c: &mut Criterion) {
    let duplicates = support::duplicate_int_vec(4_096);
    c.bench_function("find_duplicates/duplicate_int_vec", |b| {
        b.iter(|| ld::find_duplicates(black_box(&duplicates)))
    });

    let ints = support::int_vec(4_096);
    c.bench_function("find_duplicates/int_vec", |b| {
        b.iter(|| ld::find_duplicates(black_box(&ints)))
    });
}
