use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_contains_all(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).collect();
    let present = [1, 2, 3, 4];
    let missing = [1, 2, 3, 9_000];

    c.bench_function("contains_all/present", |b| {
        b.iter(|| ld::contains_all(black_box(&values), black_box(&present)))
    });

    c.bench_function("contains_all/missing", |b| {
        b.iter(|| ld::contains_all(black_box(&values), black_box(&missing)))
    });
}
