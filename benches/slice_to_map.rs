use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_slice_to_map(c: &mut Criterion) {
    let people_increasing = support::people(4_096);
    c.bench_function("slice_to_map/people/increasing", |b| {
        b.iter(|| {
            ld::slice_to_map(
                black_box(&people_increasing),
                black_box(|person: &support::Person| (person.name.clone(), person.age)),
            )
        })
    });

    let people_shuffled = support::people_shuffled(4_096);
    c.bench_function("slice_to_map/people/shuffled", |b| {
        b.iter(|| {
            ld::slice_to_map(
                black_box(&people_shuffled),
                black_box(|person: &support::Person| (person.name.clone(), person.age)),
            )
        })
    });

    let timed_records = support::timed_records(4_096);
    c.bench_function("slice_to_map/timed_records/increasing", |b| {
        b.iter(|| {
            ld::slice_to_map(
                black_box(&timed_records),
                black_box(|record: &support::TimedRecord| (record.name.clone(), record.timestamp)),
            )
        })
    });
}
