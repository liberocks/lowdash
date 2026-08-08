use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_sample_variance(c: &mut Criterion) {
    let values: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    c.bench_function("sample_variance/large", |b| {
        b.iter(|| ld::sample_variance(black_box(&values)))
    });
}
