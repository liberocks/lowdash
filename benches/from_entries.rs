use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_from_entries(c: &mut Criterion) {
    let entries = support::entry_vec(2_048);
    c.bench_function("from_entries/medium", |b| b.iter(|| ld::from_entries(black_box(&entries))));

    let small = support::entry_vec(64);
    c.bench_function("from_entries/small", |b| b.iter(|| ld::from_entries(black_box(&small))));
}
