use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_substring(c: &mut Criterion) {
    let input = support::substring_input();
    c.bench_function("substring/input/neg", |b| {
        b.iter(|| ld::substring(black_box(input), black_box(-24), black_box(18)))
    });

    c.bench_function("substring/input/pos", |b| {
        b.iter(|| ld::substring(black_box(input), black_box(2), black_box(10)))
    });

    let sentence = support::long_sentence();
    c.bench_function("substring/sentence/pos", |b| {
        b.iter(|| ld::substring(black_box(sentence), black_box(6), black_box(20)))
    });
}
