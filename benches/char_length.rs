use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_char_length(c: &mut Criterion) {
    let sentence = support::long_sentence();
    c.bench_function("char_length/long_sentence", |b| {
        b.iter(|| ld::char_length(black_box(sentence)))
    });

    let short = support::short_string();
    c.bench_function("char_length/short", |b| {
        b.iter(|| ld::char_length(black_box(short)))
    });

    let input = support::mixed_identifier();
    c.bench_function("char_length/mixed_identifier", |b| {
        b.iter(|| ld::char_length(black_box(input)))
    });
}
