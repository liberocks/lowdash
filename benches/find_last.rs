use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_find_last(c: &mut Criterion) {
    let numbers = support::int_vec(4_096);
    c.bench_function("find_last/int_vec", |b| {
        b.iter(|| ld::find_last(black_box(&numbers), black_box(|value: &i32| *value >= 0)))
    });

    let people = support::people(4_096);
    c.bench_function("find_last/people", |b| {
        b.iter(|| {
            ld::find_last(
                black_box(&people),
                black_box(|person: &support::Person| person.age >= 40),
            )
        })
    });
}
