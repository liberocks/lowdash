use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_is_sorted_by_key(c: &mut Criterion) {
    let increasing: Vec<support::Person> = (0..4_096)
        .map(|i| support::Person {
            id: i,
            name: format!("person-{i}"),
            age: i as u32,
        })
        .collect();
    c.bench_function("is_sorted_by_key/increasing", |b| {
        b.iter(|| {
            ld::is_sorted_by_key(
                black_box(&increasing),
                black_box(|p: &support::Person| p.age),
            )
        })
    });

    let descending = support::people_descending(4_096);
    c.bench_function("is_sorted_by_key/descending", |b| {
        b.iter(|| {
            ld::is_sorted_by_key(
                black_box(&descending),
                black_box(|p: &support::Person| p.age),
            )
        })
    });

    let same = support::people_same_age(4_096);
    c.bench_function("is_sorted_by_key/equal", |b| {
        b.iter(|| ld::is_sorted_by_key(black_box(&same), black_box(|p: &support::Person| p.age)))
    });
}
