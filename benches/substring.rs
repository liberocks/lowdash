use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_substring(c: &mut Criterion) {
    let input = support::substring_input();
    c.bench_function("substring", |b| {
        b.iter(|| ld::substring(black_box(input), black_box(-24), black_box(18)))
    });
}
