use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_least_common_multiple(c: &mut Criterion) {
    let values = [12_u64, 18, 30, 42];
    c.bench_function("least_common_multiple/multiple", |b| {
        b.iter(|| ld::least_common_multiple(black_box(&values)))
    });
}
