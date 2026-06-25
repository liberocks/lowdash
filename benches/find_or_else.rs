use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_find_or_else(c: &mut Criterion) {
    let collection: Vec<i32> = (0..4_096).collect();
    let fallback = -1;
    c.bench_function("find_or_else", |b| {
        b.iter(|| {
            ld::find_or_else(black_box(&collection), black_box(&fallback), black_box(|value: &i32| *value == 4_095))
        })
    });
}
