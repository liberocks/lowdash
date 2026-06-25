use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_associate(c: &mut Criterion) {
    let collection = support::people(4_096);
    c.bench_function("associate/people/increasing", |b| {
        b.iter(|| {
            ld::associate(
                black_box(&collection),
                black_box(|person: &support::Person| (person.id, person.age)),
            )
        })
    });

    let shuffled = support::people_shuffled(4_096);
    c.bench_function("associate/people/shuffled", |b| {
        b.iter(|| {
            ld::associate(
                black_box(&shuffled),
                black_box(|person: &support::Person| (person.id, person.age)),
            )
        })
    });

    let timed_records = support::timed_records(4_096);
    c.bench_function("associate/timed_records/increasing", |b| {
        b.iter(|| {
            ld::associate(
                black_box(&timed_records),
                black_box(|record: &support::TimedRecord| (record.id, record.timestamp)),
            )
        })
    });
}
