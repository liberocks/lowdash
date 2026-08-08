use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_zip(c: &mut Criterion) {
    let numbers = support::int_vec(4_096);
    let people = support::people(4_096);
    c.bench_function("zip/equal_lengths", |b| {
        b.iter(|| ld::zip(black_box(&numbers), black_box(&people)))
    });

    let short_numbers = support::int_vec(512);
    c.bench_function("zip/short_left", |b| {
        b.iter(|| ld::zip(black_box(&short_numbers), black_box(&people)))
    });

    let empty: Vec<i32> = Vec::new();
    c.bench_function("zip/empty_left", |b| {
        b.iter(|| ld::zip(black_box(&empty), black_box(&people)))
    });
}
