use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_interpolate(c: &mut Criterion) {
    c.bench_function("interpolate/10-90-0.42", |b| {
        b.iter(|| {
            let f = ld::interpolate(black_box(10.0), black_box(90.0));
            f(black_box(0.42))
        })
    });

    c.bench_function("interpolate/0-100-0.5", |b| {
        b.iter(|| {
            let f = ld::interpolate(black_box(0.0), black_box(100.0));
            f(black_box(0.5))
        })
    });
}
