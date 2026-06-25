use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_latest_by(c: &mut Criterion) {
    let collection = support::timed_records(4_096);
    c.bench_function("latest_by", |b| {
        b.iter(|| {
            ld::latest_by(black_box(&collection), black_box(|record: &support::TimedRecord| record.timestamp))
        })
    });
}
