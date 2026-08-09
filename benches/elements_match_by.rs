use crate::support;
use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_elements_match_by(c: &mut Criterion) {
    let left = support::people(512);
    let mut right = left.clone();
    right.reverse();

    c.bench_function("elements_match_by/reordered_ages", |b| {
        b.iter(|| ld::elements_match_by(black_box(&left), black_box(&right), |person| person.age))
    });

    c.bench_function("elements_match_by/reordered_ids", |b| {
        b.iter(|| ld::elements_match_by(black_box(&left), black_box(&right), |person| person.id))
    });
}
