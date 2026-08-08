use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_quantiles(c: &mut Criterion) {
    let values: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    c.bench_function("quantiles/quartiles", |b| {
        b.iter(|| ld::quantiles(black_box(&values), black_box(4)))
    });
}
