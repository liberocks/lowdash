use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_key_by(c: &mut Criterion) {
    let collection = support::people(2_048);
    c.bench_function("key_by", |b| {
        b.iter(|| {
            ld::key_by(
                black_box(&collection),
                black_box(|person: &support::Person| person.name.clone()),
            )
        })
    });
}
