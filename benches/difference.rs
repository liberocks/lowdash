use crate::support;
use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;

pub fn benchmark_difference(c: &mut Criterion) {
    let numbers = support::int_vec(4_096);
    let excluded = support::int_vec(512);
    c.bench_function("difference/int_vec", |b| {
        b.iter(|| ld::difference(black_box(&numbers), black_box(&excluded)))
    });

    let people = support::people(4_096);
    let excluded_people = support::people(512);
    c.bench_function("difference/people", |b| {
        b.iter(|| ld::difference(black_box(&people), black_box(&excluded_people)))
    });

    let empty: Vec<i32> = Vec::new();
    c.bench_function("difference/empty_excluded", |b| {
        b.iter(|| ld::difference(black_box(&numbers), black_box(&empty)))
    });
}
