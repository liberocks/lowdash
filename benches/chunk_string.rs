use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_chunk_string(c: &mut Criterion) {
    let sentence = support::long_sentence();
    c.bench_function("chunk_string/long_sentence/12", |b| {
        b.iter(|| ld::chunk_string(black_box(sentence), black_box(12)))
    });

    c.bench_function("chunk_string/long_sentence/5", |b| {
        b.iter(|| ld::chunk_string(black_box(sentence), black_box(5)))
    });

    let short = support::short_string();
    c.bench_function("chunk_string/short/3", |b| {
        b.iter(|| ld::chunk_string(black_box(short), black_box(3)))
    });
}
