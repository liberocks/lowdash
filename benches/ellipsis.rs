use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_ellipsis(c: &mut Criterion) {
    let sentence = support::long_sentence();
    c.bench_function("ellipsis/long_sentence/48", |b| {
        b.iter(|| ld::ellipsis(black_box(sentence), black_box(48)))
    });

    c.bench_function("ellipsis/long_sentence/10", |b| {
        b.iter(|| ld::ellipsis(black_box(sentence), black_box(10)))
    });

    let short = support::short_string();
    c.bench_function("ellipsis/short/3", |b| {
        b.iter(|| ld::ellipsis(black_box(short), black_box(3)))
    });
}
