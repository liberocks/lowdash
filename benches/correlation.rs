use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_correlation(c: &mut Criterion) {
    let x: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    let y: Vec<f64> = x.iter().map(|value| value * 2.0 + 1.0).collect();
    c.bench_function("correlation/large", |b| {
        b.iter(|| ld::correlation(black_box(&x), black_box(&y)))
    });
}
