use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_values(c: &mut Criterion) {
    let maps = support::numeric_maps(8, 256);
    let refs = support::map_refs(&maps);
    c.bench_function("values", |b| {
        b.iter(|| {
            ld::values(black_box(&refs))
        })
    });
}
