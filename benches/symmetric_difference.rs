use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_symmetric_difference(c: &mut Criterion) {
    let left = support::duplicate_int_vec(2_048);
    let right = support::duplicate_int_vec(2_048);
    c.bench_function("symmetric_difference/int_vec", |b| {
        b.iter(|| ld::symmetric_difference(black_box(&left), black_box(&right)))
    });

    let people = support::people(512);
    let other_people = support::people(512);
    c.bench_function("symmetric_difference/people", |b| {
        b.iter(|| ld::symmetric_difference(black_box(&people), black_box(&other_people)))
    });
}
