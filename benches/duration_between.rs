use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_duration_between(c: &mut Criterion) {
    let start = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_000);
    let end = start + std::time::Duration::from_secs(86_400 * 7);
    c.bench_function("duration_between", |b| {
        b.iter(|| {
            ld::duration_between(black_box(start), black_box(end), black_box(ld::DurationUnit::Days))
        })
    });
}
