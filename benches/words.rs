use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_words(c: &mut Criterion) {
    let input = support::long_sentence();
    c.bench_function("words", |b| {
        b.iter(|| {
            ld::words(black_box(input))
        })
    });
}
