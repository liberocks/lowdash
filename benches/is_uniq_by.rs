use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_is_uniq_by(c: &mut Criterion) {
    let people = support::people(512);
    c.bench_function("is_uniq_by/unique_ids", |b| {
        b.iter(|| ld::is_uniq_by(black_box(&people), |person| person.id))
    });

    c.bench_function("is_uniq_by/repeated_ages", |b| {
        b.iter(|| ld::is_uniq_by(black_box(&people), |person| person.age))
    });
}
