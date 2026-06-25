use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_pascal_case(c: &mut Criterion) {
    let input = support::mixed_identifier();
    c.bench_function("pascal_case", |b| {
        b.iter(|| {
            ld::pascal_case(black_box(input))
        })
    });
}
