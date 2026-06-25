use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_uniq_by(c: &mut Criterion) {
    let collection = support::people(2_048);
    c.bench_function("uniq_by", |b| {
        b.iter(|| {
            ld::uniq_by(
                black_box(&collection),
                black_box(|person: &support::Person| person.age),
            )
        })
    });
}
