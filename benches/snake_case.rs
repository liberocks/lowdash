use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_snake_case(c: &mut Criterion) {
    let input = support::mixed_identifier();
    c.bench_function("snake_case", |b| {
        b.iter(|| {
            ld::snake_case(black_box(input))
        })
    });
}
