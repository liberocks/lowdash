use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_key_by(c: &mut Criterion) {
    let people_increasing = support::people(4_096);
    c.bench_function("key_by/people/increasing", |b| {
        b.iter(|| {
            ld::key_by(
                black_box(&people_increasing),
                black_box(|person: &support::Person| person.name.clone()),
            )
        })
    });

    let people_shuffled = support::people_shuffled(4_096);
    c.bench_function("key_by/people/shuffled", |b| {
        b.iter(|| {
            ld::key_by(
                black_box(&people_shuffled),
                black_box(|person: &support::Person| person.name.clone()),
            )
        })
    });

    let timed_records = support::timed_records(4_096);
    c.bench_function("key_by/timed_records/increasing", |b| {
        b.iter(|| {
            ld::key_by(
                black_box(&timed_records),
                black_box(|record: &support::TimedRecord| record.name.clone()),
            )
        })
    });
}
