use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_product_with_start(c: &mut Criterion) {
    let values = vec![1.0001_f64; 4_096];
    c.bench_function("product_with_start/large", |b| {
        b.iter(|| ld::product_with_start(black_box(&values), black_box(1.25_f64)))
    });
}
