use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_every(c: &mut Criterion) {
    let numbers = support::int_vec(4_096);
    c.bench_function("every/early_failure", |b| {
        b.iter(|| ld::every(black_box(&numbers), black_box(|value: &i32| *value < 10)))
    });

    let positive: Vec<i32> = (1..4_097).collect();
    c.bench_function("every/all_match", |b| {
        b.iter(|| ld::every(black_box(&positive), black_box(|value: &i32| *value > 0)))
    });

    let people = support::people(4_096);
    c.bench_function("every/people", |b| {
        b.iter(|| {
            ld::every(
                black_box(&people),
                black_box(|person: &support::Person| person.age >= 18),
            )
        })
    });
}
