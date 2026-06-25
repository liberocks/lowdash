use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_uniq_keys(c: &mut Criterion) {
    let maps = support::numeric_maps(8, 256);
    let refs = support::map_refs(&maps);
    c.bench_function("uniq_keys/medium", |b| b.iter(|| ld::uniq_keys(black_box(&refs))));

    let small_maps = support::numeric_maps(4, 32);
    let small_refs = support::map_refs(&small_maps);
    c.bench_function("uniq_keys/small", |b| b.iter(|| ld::uniq_keys(black_box(&small_refs))));
}
