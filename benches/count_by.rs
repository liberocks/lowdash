use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_count_by(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("count_by/int_vec", |b| {
        b.iter(|| ld::count_by(black_box(&ints), black_box(|value: &i32| *value > 0)))
    });

    let duplicates = support::duplicate_int_vec(4_096);
    c.bench_function("count_by/duplicate_int_vec", |b| {
        b.iter(|| ld::count_by(black_box(&duplicates), black_box(|value: &i32| *value > 0)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("count_by/float_vec", |b| {
        b.iter(|| ld::count_by(black_box(&floats), black_box(|value: &f64| *value > 0.0)))
    });

    let people = support::people(4_096);
    c.bench_function("count_by/people", |b| {
        b.iter(|| ld::count_by(black_box(&people), black_box(|p: &support::Person| p.age > 30)))
    });
}
