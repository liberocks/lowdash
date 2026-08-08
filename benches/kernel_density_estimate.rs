use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_kernel_density_estimate(c: &mut Criterion) {
    let values: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    let estimate = ld::kernel_density_estimate(&values, 0.5).unwrap();
    c.bench_function("kernel_density_estimate/large", |b| {
        b.iter(|| estimate(black_box(1.0)))
    });
}
