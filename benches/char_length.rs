use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_char_length(c: &mut Criterion) {
    let input = support::long_sentence();
    c.bench_function("char_length", |b| {
        b.iter(|| {
            ld::char_length(black_box(input))
        })
    });
}
