use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_earliest_by(c: &mut Criterion) {
    let collection = support::timed_records(4_096);
    c.bench_function("earliest_by", |b| {
        b.iter(|| {
            ld::earliest_by(
                black_box(&collection),
                black_box(|record: &support::TimedRecord| record.timestamp),
            )
        })
    });
}
