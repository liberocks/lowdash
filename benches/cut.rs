use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_cut(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).collect();
    let separator = [2_048, 2_049, 2_050];

    c.bench_function("cut/int_vec/middle_separator", |b| {
        b.iter(|| ld::cut(black_box(&values), black_box(&separator)))
    });

    c.bench_function("cut/int_vec/missing_separator", |b| {
        b.iter(|| ld::cut(black_box(&values), black_box(&[9_000, 9_001])))
    });
}
