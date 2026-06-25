use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_chunk_string(c: &mut Criterion) {
    let input = support::long_sentence();
    c.bench_function("chunk_string", |b| {
        b.iter(|| {
            ld::chunk_string(black_box(input), black_box(12))
        })
    });
}
