use criterion::{black_box, Criterion};
use crate::support;
use lowdash as ld;

pub fn benchmark_subset(c: &mut Criterion) {
    let collection = support::int_vec(4_096);
    c.bench_function("subset", |b| {
        b.iter(|| {
            ld::subset(black_box(&collection), black_box(-512), black_box(256))
        })
    });
}
