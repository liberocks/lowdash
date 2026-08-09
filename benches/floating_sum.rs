use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_floating_sum(c: &mut Criterion) {
    let values: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    c.bench_function("floating_sum/large", |b| {
        b.iter(|| ld::floating_sum(black_box(&values)))
    });
}
