use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_uniq(c: &mut Criterion) {
    let duplicates = support::duplicate_int_vec(4_096);
    c.bench_function("uniq/duplicate_int_vec", |b| {
        b.iter(|| ld::uniq(black_box(&duplicates)))
    });

    let ints = support::int_vec(4_096);
    c.bench_function("uniq/int_vec", |b| b.iter(|| ld::uniq(black_box(&ints))));

    let floats = support::float_vec(4_096);
    c.bench_function("uniq/float_vec", |b| {
        b.iter(|| ld::uniq(black_box(&floats)))
    });
}
