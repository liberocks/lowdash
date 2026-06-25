use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_max_by(c: &mut Criterion) {
    let collection = support::people(4_096);
    c.bench_function("max_by", |b| {
        b.iter(|| {
            ld::max_by(
                black_box(&collection),
                black_box(|a: &support::Person, b: &support::Person| a.age > b.age),
            )
        })
    });
}
