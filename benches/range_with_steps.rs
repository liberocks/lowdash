use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_range_with_steps(c: &mut Criterion) {
    c.bench_function("range_with_steps", |b| {
        b.iter(|| {
            ld::range_with_steps(black_box(0_i32), black_box(10_000_i32), black_box(3_i32))
        })
    });
}
