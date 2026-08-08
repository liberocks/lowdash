use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_median_grouped(c: &mut Criterion) {
    let values: Vec<f64> = (0..4_096).map(|value| (value % 32) as f64).collect();
    c.bench_function("median_grouped/large", |b| {
        b.iter(|| ld::median_grouped(black_box(&values), black_box(1.0)))
    });
}
