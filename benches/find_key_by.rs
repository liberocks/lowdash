use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_find_key_by(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    c.bench_function("find_key_by/map", |b| {
        b.iter(|| {
            ld::find_key_by(
                black_box(&map),
                black_box(|key: &String, value: &i32| key.ends_with('7') && *value >= 0),
            )
        })
    });

    let map_large = support::numeric_map(8_192);
    c.bench_function("find_key_by/large", |b| {
        b.iter(|| {
            ld::find_key_by(
                black_box(&map_large),
                black_box(|key: &String, value: &i32| key.ends_with('7') && *value >= 0),
            )
        })
    });
}
