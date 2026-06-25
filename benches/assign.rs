use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_assign(c: &mut Criterion) {
    let maps = support::numeric_maps(8, 256);
    c.bench_function("assign", |b| {
        b.iter(|| {
            ld::assign(black_box(&maps))
        })
    });
}
