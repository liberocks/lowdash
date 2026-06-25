use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_invert(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    c.bench_function("invert", |b| {
        b.iter(|| {
            ld::invert(black_box(&map))
        })
    });
}
