use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_median_high(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).rev().collect();
    c.bench_function("median_high/large", |b| {
        b.iter(|| ld::median_high(black_box(&values)))
    });
}
