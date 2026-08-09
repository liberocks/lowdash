use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_contains_any(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).collect();
    let present = [4_000, 4_001, 4_002];
    let missing = [9_000, 9_001, 9_002];

    c.bench_function("contains_any/present", |b| {
        b.iter(|| ld::contains_any(black_box(&values), black_box(&present)))
    });

    c.bench_function("contains_any/missing", |b| {
        b.iter(|| ld::contains_any(black_box(&values), black_box(&missing)))
    });
}
