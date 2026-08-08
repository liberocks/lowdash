use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_floating_mean(c: &mut Criterion) {
    let values: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    c.bench_function("floating_mean/large", |b| {
        b.iter(|| ld::floating_mean(black_box(&values)))
    });
}
