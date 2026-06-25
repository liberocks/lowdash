use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_common_random_usize_with_seed(c: &mut Criterion) {
    c.bench_function("common_random_usize_with_seed", |b| {
        b.iter(|| ld::common::random_usize_with_seed(black_box(10_000), black_box(0x5eed_u64)))
    });
}
