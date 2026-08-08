use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_intersection_by(c: &mut Criterion) {
    let collections = vec![
        support::int_vec(2_048),
        support::int_vec(2_048),
        support::int_vec(2_048),
    ];
    c.bench_function("intersection_by/int_vec", |b| {
        b.iter(|| {
            ld::intersection_by(
                black_box(&collections),
                black_box(|value: &i32| *value % 97),
            )
        })
    });

    let people = vec![
        support::people(512),
        support::people(512),
        support::people(512),
    ];
    c.bench_function("intersection_by/people", |b| {
        b.iter(|| {
            ld::intersection_by(
                black_box(&people),
                black_box(|person: &support::Person| person.age),
            )
        })
    });
}
