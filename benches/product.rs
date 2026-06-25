use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_product(c: &mut Criterion) {
    let collection: Vec<f64> = (0..128).map(|i| 1.0 + (i % 5) as f64 / 100.0).collect();
    c.bench_function("product", |b| {
        b.iter(|| ld::product(black_box(&collection)))
    });
}
