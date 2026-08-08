use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_cartesian_product(c: &mut Criterion) {
    let left = support::int_vec(128);
    let right = support::int_vec(128);
    c.bench_function("cartesian_product/128x128", |b| {
        b.iter(|| ld::cartesian_product(black_box(&left), black_box(&right)))
    });

    let short_left = support::int_vec(16);
    c.bench_function("cartesian_product/16x128", |b| {
        b.iter(|| ld::cartesian_product(black_box(&short_left), black_box(&right)))
    });
}
