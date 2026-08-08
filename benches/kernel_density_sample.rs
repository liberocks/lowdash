use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_kernel_density_sample(c: &mut Criterion) {
    let values: Vec<f64> = (0..4_096).map(|value| value as f64 / 3.0).collect();
    c.bench_function("kernel_density_sample/large", |b| {
        b.iter(|| {
            ld::kernel_density_sample(
                black_box(&values),
                black_box(0.5),
                black_box(4_096),
                black_box(42),
            )
        })
    });
}
