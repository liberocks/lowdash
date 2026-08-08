use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_geometric_mean(c: &mut Criterion) {
    let values: Vec<f64> = (1..=4_096).map(|value| value as f64 / 3.0).collect();
    c.bench_function("geometric_mean/large", |b| {
        b.iter(|| ld::geometric_mean(black_box(&values)))
    });
}
