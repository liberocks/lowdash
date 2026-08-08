use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_assign_with(c: &mut Criterion) {
    let small = support::numeric_maps(8, 256);
    c.bench_function("assign_with/small", |b| {
        b.iter(|| {
            ld::assign_with(
                black_box(&small),
                black_box(|_, existing: &i32, incoming: &i32| existing + incoming),
            )
        })
    });

    let large = support::numeric_maps(32, 1_024);
    c.bench_function("assign_with/large", |b| {
        b.iter(|| {
            ld::assign_with(
                black_box(&large),
                black_box(|_, existing: &i32, incoming: &i32| existing + incoming),
            )
        })
    });
}
