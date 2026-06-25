use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_filter(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("filter/int_vec", |b| {
        b.iter(|| ld::filter(black_box(&ints), black_box(|value: &i32, _| *value % 2 == 0)))
    });

    let duplicates = support::duplicate_int_vec(4_096);
    c.bench_function("filter/duplicate_int_vec", |b| {
        b.iter(|| ld::filter(black_box(&duplicates), black_box(|value: &i32, _| *value % 2 == 0)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("filter/float_vec", |b| {
        b.iter(|| ld::filter(black_box(&floats), black_box(|value: &f64, _| *value > 0.0)))
    });
}
