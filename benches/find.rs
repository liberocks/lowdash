use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_find(c: &mut Criterion) {
    let collection: Vec<i32> = (0..4_096).collect();
    c.bench_function("find", |b| {
        b.iter(|| {
            ld::find(
                black_box(&collection),
                black_box(|value: &i32| *value == 4_095),
            )
        })
    });
}
