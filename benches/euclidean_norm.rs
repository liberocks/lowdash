use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_euclidean_norm(c: &mut Criterion) {
    let values: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    c.bench_function("euclidean_norm/large", |b| {
        b.iter(|| ld::euclidean_norm(black_box(&values)))
    });
}
