use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_intersection(c: &mut Criterion) {
    let collections = vec![
        support::int_vec(4_096),
        support::int_vec(4_096),
        support::int_vec(4_096),
    ];
    c.bench_function("intersection/int_vec", |b| {
        b.iter(|| ld::intersection(black_box(&collections)))
    });

    let people = vec![
        support::people(1_024),
        support::people(1_024),
        support::people(1_024),
    ];
    c.bench_function("intersection/people", |b| {
        b.iter(|| ld::intersection(black_box(&people)))
    });

    let empty: Vec<Vec<i32>> = Vec::new();
    c.bench_function("intersection/empty", |b| {
        b.iter(|| ld::intersection::<i32, Vec<i32>>(black_box(&empty)))
    });
}
