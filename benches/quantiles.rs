use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_quantiles(c: &mut Criterion) {
    let values: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    c.bench_function("quantiles/quartiles", |b| {
        b.iter(|| ld::quantiles(black_box(&values), black_box(4)))
    });
}
