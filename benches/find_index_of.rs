use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_find_index_of(c: &mut Criterion) {
    let collection: Vec<i32> = (0..4_096).collect();
    c.bench_function("find_index_of", |b| {
        b.iter(|| {
            ld::find_index_of(black_box(&collection), black_box(|value: &i32| *value == 4_095))
        })
    });
}
