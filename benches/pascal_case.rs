use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_pascal_case(c: &mut Criterion) {
    let input = support::mixed_identifier();
    c.bench_function("pascal_case/mixed_identifier", |b| {
        b.iter(|| ld::pascal_case(black_box(input)))
    });

    let sentence = support::long_sentence();
    c.bench_function("pascal_case/long_sentence", |b| {
        b.iter(|| ld::pascal_case(black_box(sentence)))
    });

    let short = support::short_string();
    c.bench_function("pascal_case/short", |b| {
        b.iter(|| ld::pascal_case(black_box(short)))
    });
}
