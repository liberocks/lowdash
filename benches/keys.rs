use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_keys(c: &mut Criterion) {
    let maps = support::numeric_maps(8, 256);
    let refs = support::map_refs(&maps);
    c.bench_function("keys", |b| b.iter(|| ld::keys(black_box(&refs))));
}
