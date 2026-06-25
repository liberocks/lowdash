use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_pick_by(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    c.bench_function("pick_by", |b| {
        b.iter(|| {
            ld::pick_by(
                black_box(&map),
                black_box(|_: &String, value: &i32| *value > 0),
            )
        })
    });
}
