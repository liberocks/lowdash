use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_map(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("map/int_vec", |b| {
        b.iter(|| ld::map(black_box(&ints), black_box(|value: &i32, _| *value * 2)))
    });

    let duplicates = support::duplicate_int_vec(4_096);
    c.bench_function("map/duplicate_int_vec", |b| {
        b.iter(|| {
            ld::map(
                black_box(&duplicates),
                black_box(|value: &i32, _| *value * 2),
            )
        })
    });

    let floats = support::float_vec(4_096);
    c.bench_function("map/float_vec", |b| {
        b.iter(|| ld::map(black_box(&floats), black_box(|value: &f64, _| *value * 2.0)))
    });
}
