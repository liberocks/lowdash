use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_max_by(c: &mut Criterion) {
    let people_increasing = support::people(4_096);
    c.bench_function("max_by/people/increasing", |b| {
        b.iter(|| {
            ld::max_by(
                black_box(&people_increasing),
                black_box(|a: &support::Person, b: &support::Person| a.age > b.age),
            )
        })
    });

    let people_desc = support::people_descending(4_096);
    c.bench_function("max_by/people/descending", |b| {
        b.iter(|| {
            ld::max_by(
                black_box(&people_desc),
                black_box(|a: &support::Person, b: &support::Person| a.age > b.age),
            )
        })
    });

    let people_same = support::people_same_age(4_096);
    c.bench_function("max_by/people/equal", |b| {
        b.iter(|| {
            ld::max_by(
                black_box(&people_same),
                black_box(|a: &support::Person, b: &support::Person| a.age > b.age),
            )
        })
    });

    let people_shuffled = support::people_shuffled(4_096);
    c.bench_function("max_by/people/shuffled", |b| {
        b.iter(|| {
            ld::max_by(
                black_box(&people_shuffled),
                black_box(|a: &support::Person, b: &support::Person| a.age > b.age),
            )
        })
    });

    let timed_records = support::timed_records(4_096);
    c.bench_function("max_by/timed_records/increasing", |b| {
        b.iter(|| {
            ld::max_by(
                black_box(&timed_records),
                black_box(|a: &support::TimedRecord, b: &support::TimedRecord| {
                    a.timestamp > b.timestamp
                }),
            )
        })
    });

    let copy_timed_records = support::copy_timed_records(4_096);
    c.bench_function("max_by/copy_timed_records/increasing", |b| {
        b.iter(|| {
            ld::max_by(
                black_box(&copy_timed_records),
                black_box(
                    |a: &support::CopyTimedRecord, b: &support::CopyTimedRecord| {
                        a.timestamp > b.timestamp
                    },
                ),
            )
        })
    });
}
