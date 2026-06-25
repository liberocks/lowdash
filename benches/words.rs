use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_words(c: &mut Criterion) {
    let sentence = support::long_sentence();
    c.bench_function("words/long_sentence", |b| b.iter(|| ld::words(black_box(sentence))));

    let input = support::mixed_identifier();
    c.bench_function("words/mixed_identifier", |b| b.iter(|| ld::words(black_box(input))));

    let short = support::short_string();
    c.bench_function("words/short", |b| b.iter(|| ld::words(black_box(short))));
}
