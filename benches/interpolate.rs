use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_interpolate(c: &mut Criterion) {
    c.bench_function("interpolate", |b| {
        b.iter(|| {
            let f = ld::interpolate(black_box(10.0), black_box(90.0));
            f(black_box(0.42))
        })
    });
}
