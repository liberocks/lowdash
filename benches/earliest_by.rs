use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_earliest_by(c: &mut Criterion) {
    let timed_records_increasing = support::timed_records(4_096);
    c.bench_function("earliest_by/timed_records/increasing", |b| {
        b.iter(|| {
            ld::earliest_by(
                black_box(&timed_records_increasing),
                black_box(|record: &support::TimedRecord| record.timestamp),
            )
        })
    });

    let timed_records_descending = support::timed_records_descending(4_096);
    c.bench_function("earliest_by/timed_records/descending", |b| {
        b.iter(|| {
            ld::earliest_by(
                black_box(&timed_records_descending),
                black_box(|record: &support::TimedRecord| record.timestamp),
            )
        })
    });

    let timed_records_equal = support::timed_records_equal(4_096);
    c.bench_function("earliest_by/timed_records/equal", |b| {
        b.iter(|| {
            ld::earliest_by(
                black_box(&timed_records_equal),
                black_box(|record: &support::TimedRecord| record.timestamp),
            )
        })
    });

    let timed_records_shuffled = support::timed_records_shuffled(4_096);
    c.bench_function("earliest_by/timed_records/shuffled", |b| {
        b.iter(|| {
            ld::earliest_by(
                black_box(&timed_records_shuffled),
                black_box(|record: &support::TimedRecord| record.timestamp),
            )
        })
    });

    let copy_records_increasing = support::copy_timed_records(4_096);
    c.bench_function("earliest_by/copy_timed_records/increasing", |b| {
        b.iter(|| {
            ld::earliest_by(
                black_box(&copy_records_increasing),
                black_box(|record: &support::CopyTimedRecord| record.timestamp),
            )
        })
    });

    let copy_records_descending = support::copy_timed_records_descending(4_096);
    c.bench_function("earliest_by/copy_timed_records/descending", |b| {
        b.iter(|| {
            ld::earliest_by(
                black_box(&copy_records_descending),
                black_box(|record: &support::CopyTimedRecord| record.timestamp),
            )
        })
    });

    let copy_records_equal = support::copy_timed_records_equal(4_096);
    c.bench_function("earliest_by/copy_timed_records/equal", |b| {
        b.iter(|| {
            ld::earliest_by(
                black_box(&copy_records_equal),
                black_box(|record: &support::CopyTimedRecord| record.timestamp),
            )
        })
    });

    let copy_records_shuffled = support::copy_timed_records_shuffled(4_096);
    c.bench_function("earliest_by/copy_timed_records/shuffled", |b| {
        b.iter(|| {
            ld::earliest_by(
                black_box(&copy_records_shuffled),
                black_box(|record: &support::CopyTimedRecord| record.timestamp),
            )
        })
    });
}
