use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_repeat(c: &mut Criterion) {
    c.bench_function("repeat", |b| {
        b.iter(|| {
            ld::repeat(black_box(2_048), black_box(7_i32))
        })
    });
}
