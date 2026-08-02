use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_take_right_while(c: &mut Criterion) {
    let numbers: Vec<i32> = (0..4_096).collect();
    c.bench_function("take_right_while/half", |b| {
        b.iter(|| {
            ld::take_right_while(
                black_box(&numbers),
                black_box(|value: &i32| *value >= 2_048),
            )
        })
    });

    let people = support::people(4_096);
    c.bench_function("take_right_while/people", |b| {
        b.iter(|| {
            ld::take_right_while(
                black_box(&people),
                black_box(|person: &support::Person| person.id >= 2_048),
            )
        })
    });

    let empty: Vec<i32> = Vec::new();
    c.bench_function("take_right_while/empty", |b| {
        b.iter(|| ld::take_right_while(black_box(&empty), black_box(|_: &i32| true)))
    });
}
