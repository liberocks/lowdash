use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_subset(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    c.bench_function("subset/int_vec/neg", |b| {
        b.iter(|| ld::subset(black_box(&ints), black_box(-512), black_box(256)))
    });

    c.bench_function("subset/int_vec/pos", |b| {
        b.iter(|| ld::subset(black_box(&ints), black_box(100), black_box(200)))
    });

    let floats = support::float_vec(4_096);
    c.bench_function("subset/float_vec/neg", |b| {
        b.iter(|| ld::subset(black_box(&floats), black_box(-512), black_box(256)))
    });
}
