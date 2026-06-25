use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_slice_to_map(c: &mut Criterion) {
    let collection = support::people(2_048);
    c.bench_function("slice_to_map", |b| {
        b.iter(|| {
            ld::slice_to_map(
                black_box(&collection),
                black_box(|person: &support::Person| (person.name.clone(), person.age)),
            )
        })
    });
}
