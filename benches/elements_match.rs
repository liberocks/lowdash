use crate::support;
use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_elements_match(c: &mut Criterion) {
    let left = support::duplicate_int_vec(512);
    let mut right = left.clone();
    right.reverse();

    c.bench_function("elements_match/reordered_duplicates", |b| {
        b.iter(|| ld::elements_match(black_box(&left), black_box(&right)))
    });

    let different = support::int_vec(512);
    c.bench_function("elements_match/different_inputs", |b| {
        b.iter(|| ld::elements_match(black_box(&left), black_box(&different)))
    });
}
