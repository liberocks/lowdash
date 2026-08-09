use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_integer_square_root(c: &mut Criterion) {
    c.bench_function("integer_square_root/u128-max", |b| {
        b.iter(|| ld::integer_square_root(black_box(u128::MAX)))
    });
}
