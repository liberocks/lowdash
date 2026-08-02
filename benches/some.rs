use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_some(c: &mut Criterion) {
    let numbers = support::int_vec(4_096);
    c.bench_function("some/early_match", |b| {
        b.iter(|| ld::some(black_box(&numbers), black_box(|value: &i32| *value == -48)))
    });

    let negative: Vec<i32> = (1..4_097).map(|value| -value).collect();
    c.bench_function("some/no_match", |b| {
        b.iter(|| ld::some(black_box(&negative), black_box(|value: &i32| *value > 0)))
    });

    let people = support::people(4_096);
    c.bench_function("some/people", |b| {
        b.iter(|| {
            ld::some(
                black_box(&people),
                black_box(|person: &support::Person| person.age >= 60),
            )
        })
    });
}
