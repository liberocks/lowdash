use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_greatest_common_divisor(c: &mut Criterion) {
    let values = [48_u64, 18, 30, 42, 66];
    c.bench_function("greatest_common_divisor/multiple", |b| {
        b.iter(|| ld::greatest_common_divisor(black_box(&values)))
    });
}
