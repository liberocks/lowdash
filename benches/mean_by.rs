use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_mean_by(c: &mut Criterion) {
    let people_increasing = support::people(4_096);
    c.bench_function("mean_by/people/increasing", |b| {
        b.iter(|| {
            ld::mean_by(
                black_box(&people_increasing),
                black_box(|person: &support::Person| person.age as f64),
            )
        })
    });

    let people_same = support::people_same_age(4_096);
    c.bench_function("mean_by/people/equal", |b| {
        b.iter(|| {
            ld::mean_by(
                black_box(&people_same),
                black_box(|person: &support::Person| person.age as f64),
            )
        })
    });

    let ints = support::int_vec(4_096);
    c.bench_function("mean_by/int_vec", |b| {
        b.iter(|| ld::mean_by(black_box(&ints), black_box(|x: &i32| *x as f64)))
    });
}
