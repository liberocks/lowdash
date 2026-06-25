use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_combination(c: &mut Criterion) {
    let items: Vec<i32> = (0..18).collect();
    c.bench_function("combination", |b| {
        b.iter(|| ld::combination(black_box(&items), black_box(3)))
    });
}
