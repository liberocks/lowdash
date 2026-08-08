use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_take_while(c: &mut Criterion) {
    let numbers = support::int_vec(4_096);
    c.bench_function("take_while/early_stop", |b| {
        b.iter(|| ld::take_while(black_box(&numbers), black_box(|value: &i32| *value < 10)))
    });

    let ascending: Vec<i32> = (0..4_096).collect();
    c.bench_function("take_while/half", |b| {
        b.iter(|| {
            ld::take_while(
                black_box(&ascending),
                black_box(|value: &i32| *value < 2_048),
            )
        })
    });

    let people = support::people(4_096);
    c.bench_function("take_while/people", |b| {
        b.iter(|| {
            ld::take_while(
                black_box(&people),
                black_box(|person: &support::Person| person.age < 30),
            )
        })
    });
}
