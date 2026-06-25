use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_value_or(c: &mut Criterion) {
    let map = support::numeric_map(2_048);
    let missing = String::from("missing");
    c.bench_function("value_or/map/missing", |b| {
        b.iter(|| ld::value_or(black_box(&map), black_box(&missing), black_box(-1)))
    });

    let existing = String::from("key-1024");
    c.bench_function("value_or/map/existing", |b| {
        b.iter(|| ld::value_or(black_box(&map), black_box(&existing), black_box(-1)))
    });
}
