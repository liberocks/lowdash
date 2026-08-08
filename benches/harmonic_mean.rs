use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_harmonic_mean(c: &mut Criterion) {
    let values: Vec<f64> = (1..=4_096).map(|value| value as f64 / 3.0).collect();
    c.bench_function("harmonic_mean/large", |b| {
        b.iter(|| ld::harmonic_mean(black_box(&values)))
    });
}
