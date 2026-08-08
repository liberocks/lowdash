use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_union(c: &mut Criterion) {
    let collections = vec![
        support::duplicate_int_vec(1_024),
        support::duplicate_int_vec(1_024),
        support::duplicate_int_vec(1_024),
    ];
    c.bench_function("union/duplicate_int_vec", |b| {
        b.iter(|| ld::union(black_box(&collections)))
    });

    let people = vec![
        support::people(512),
        support::people(512),
        support::people(512),
    ];
    c.bench_function("union/people", |b| b.iter(|| ld::union(black_box(&people))));

    let empty: Vec<Vec<i32>> = Vec::new();
    c.bench_function("union/empty", |b| {
        b.iter(|| ld::union::<i32, Vec<i32>>(black_box(&empty)))
    });
}
