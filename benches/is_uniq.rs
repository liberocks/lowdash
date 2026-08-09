use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_is_uniq(c: &mut Criterion) {
    let unique = support::int_vec(512);
    c.bench_function("is_uniq/unique_int_vec", |b| {
        b.iter(|| ld::is_uniq(black_box(&unique)))
    });

    let duplicates = support::duplicate_int_vec(512);
    c.bench_function("is_uniq/duplicate_int_vec", |b| {
        b.iter(|| ld::is_uniq(black_box(&duplicates)))
    });
}
