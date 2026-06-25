use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_associate(c: &mut Criterion) {
    let collection = support::people(2_048);
    c.bench_function("associate", |b| {
        b.iter(|| {
            ld::associate(
                black_box(&collection),
                black_box(|person: &support::Person| (person.id, person.age)),
            )
        })
    });
}
