use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_min_by(c: &mut Criterion) {
    let collection = support::people(4_096);
    c.bench_function("min_by", |b| {
        b.iter(|| {
            ld::min_by(
                black_box(&collection),
                black_box(|a: &support::Person, b: &support::Person| a.age < b.age),
            )
        })
    });
}
