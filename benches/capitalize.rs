use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_capitalize(c: &mut Criterion) {
    let input = support::long_sentence();
    c.bench_function("capitalize", |b| {
        b.iter(|| {
            ld::capitalize(black_box(input))
        })
    });
}
