use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_permutation_count(c: &mut Criterion) {
    c.bench_function("permutation_count/52-permute-5", |b| {
        b.iter(|| ld::permutation_count(black_box(52), black_box(5)))
    });
}
