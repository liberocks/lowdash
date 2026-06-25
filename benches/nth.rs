use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_nth(c: &mut Criterion) {
    let collection: Vec<i32> = (0..4_096).collect();
    c.bench_function("nth", |b| {
        b.iter(|| ld::nth(black_box(&collection), black_box(-10)))
    });
}
