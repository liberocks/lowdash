use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_omit_by(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    c.bench_function("omit_by", |b| {
        b.iter(|| {
            ld::omit_by(black_box(&map), black_box(|_: &String, value: &i32| *value > 0))
        })
    });
}
