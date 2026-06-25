use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_sum(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("sum/int_vec", |b| b.iter(|| ld::sum(black_box(&ints))));

    let floats = support::float_vec(4_096);
    c.bench_function("sum/float_vec", |b| b.iter(|| ld::sum(black_box(&floats))));

    let duplicates = support::duplicate_int_vec(4_096);
    c.bench_function("sum/duplicate_int_vec", |b| {
        b.iter(|| ld::sum(black_box(&duplicates)))
    });
}
