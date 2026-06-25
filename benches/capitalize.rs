use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_capitalize(c: &mut Criterion) {
    let sentence = support::long_sentence();
    c.bench_function("capitalize/long_sentence", |b| {
        b.iter(|| ld::capitalize(black_box(sentence)))
    });

    let short = support::short_string();
    c.bench_function("capitalize/short", |b| {
        b.iter(|| ld::capitalize(black_box(short)))
    });

    let input = support::mixed_identifier();
    c.bench_function("capitalize/mixed_identifier", |b| {
        b.iter(|| ld::capitalize(black_box(input)))
    });
}
