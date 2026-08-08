use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_population_standard_deviation(c: &mut Criterion) {
    let values: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    c.bench_function("population_standard_deviation/large", |b| {
        b.iter(|| ld::population_standard_deviation(black_box(&values)))
    });
}
