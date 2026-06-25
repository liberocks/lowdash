use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_kebab_case(c: &mut Criterion) {
    let input = support::mixed_identifier();
    c.bench_function("kebab_case", |b| {
        b.iter(|| ld::kebab_case(black_box(input)))
    });
}
