use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_product_by(c: &mut Criterion) {
    let collection = support::people(256);
    c.bench_function("product_by", |b| {
        b.iter(|| {
            ld::product_by(
                black_box(&collection),
                black_box(|person: &support::Person| 1.0 + (person.age % 3) as f64 / 100.0),
            )
        })
    });
}
