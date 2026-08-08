use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_sort_by(c: &mut Criterion) {
    let numbers = support::int_vec_shuffled(4_096);
    c.bench_function("sort_by/int_vec_shuffled", |b| {
        b.iter(|| {
            ld::sort_by(
                black_box(&numbers),
                black_box(|left: &i32, right: &i32| left.cmp(right)),
            )
        })
    });

    let people = support::people_shuffled(4_096);
    c.bench_function("sort_by/people_descending_age", |b| {
        b.iter(|| {
            ld::sort_by(
                black_box(&people),
                black_box(|left: &support::Person, right: &support::Person| {
                    right.age.cmp(&left.age)
                }),
            )
        })
    });

    let equal_age_people = support::people_same_age(4_096);
    c.bench_function("sort_by/equal_items", |b| {
        b.iter(|| {
            ld::sort_by(
                black_box(&equal_age_people),
                black_box(|left: &support::Person, right: &support::Person| {
                    left.age.cmp(&right.age)
                }),
            )
        })
    });
}
