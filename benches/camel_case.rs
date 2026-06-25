use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_camel_case(c: &mut Criterion) {
    let input = support::mixed_identifier();
    c.bench_function("camel_case", |b| {
        b.iter(|| {
            ld::camel_case(black_box(input))
        })
    });
}
