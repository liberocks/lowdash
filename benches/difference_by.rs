use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_difference_by(c: &mut Criterion) {
    let numbers = support::int_vec(4_096);
    let excluded = support::int_vec(512);
    c.bench_function("difference_by/int_vec", |b| {
        b.iter(|| {
            ld::difference_by(
                black_box(&numbers),
                black_box(&excluded),
                black_box(|value: &i32| *value % 97),
            )
        })
    });

    let people = support::people(2_048);
    let excluded_people = support::people(256);
    c.bench_function("difference_by/people", |b| {
        b.iter(|| {
            ld::difference_by(
                black_box(&people),
                black_box(&excluded_people),
                black_box(|person: &support::Person| person.age),
            )
        })
    });
}
