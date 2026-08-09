use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_covariance(c: &mut Criterion) {
    let x: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    let y: Vec<f64> = x.iter().map(|value| value * 2.0).collect();
    c.bench_function("covariance/large", |b| {
        b.iter(|| ld::covariance(black_box(&x), black_box(&y)))
    });
}
