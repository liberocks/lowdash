use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_values(c: &mut Criterion) {
    let maps = support::numeric_maps(8, 256);
    let refs = support::map_refs(&maps);
    c.bench_function("values/medium", |b| b.iter(|| ld::values(black_box(&refs))));

    let small_maps = support::numeric_maps(4, 32);
    let small_refs = support::map_refs(&small_maps);
    c.bench_function("values/small", |b| {
        b.iter(|| ld::values(black_box(&small_refs)))
    });
}
