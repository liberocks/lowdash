use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_sum_products(c: &mut Criterion) {
    let p: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    let q: Vec<f64> = p.iter().map(|value| value + 1.0).collect();
    c.bench_function("sum_products/large", |b| {
        b.iter(|| ld::sum_products(black_box(&p), black_box(&q)))
    });
}
