use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_weighted_mean(c: &mut Criterion) {
    let values: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    let weights: Vec<f64> = (0..4_096).map(|value| (value % 17 + 1) as f64).collect();
    c.bench_function("weighted_mean/large", |b| {
        b.iter(|| ld::weighted_mean(black_box(&values), black_box(&weights)))
    });
}
