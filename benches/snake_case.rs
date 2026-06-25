use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_snake_case(c: &mut Criterion) {
    let input = support::mixed_identifier();
    c.bench_function("snake_case/mixed_identifier", |b| {
        b.iter(|| ld::snake_case(black_box(input)))
    });

    let sentence = support::long_sentence();
    c.bench_function("snake_case/long_sentence", |b| {
        b.iter(|| ld::snake_case(black_box(sentence)))
    });

    let short = support::short_string();
    c.bench_function("snake_case/short", |b| {
        b.iter(|| ld::snake_case(black_box(short)))
    });
}
