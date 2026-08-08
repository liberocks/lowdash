use crate::support;
use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;

pub fn benchmark_sort_by_key(c: &mut Criterion) {
    let people = support::people_shuffled(4_096);
    c.bench_function("sort_by_key/people_shuffled", |b| {
        b.iter(|| {
            ld::sort_by_key(
                black_box(&people),
                black_box(|person: &support::Person| person.age),
            )
        })
    });

    let numbers = support::int_vec_shuffled(4_096);
    c.bench_function("sort_by_key/int_vec_shuffled", |b| {
        b.iter(|| ld::sort_by_key(black_box(&numbers), black_box(|number: &i32| *number)))
    });

    let equal_age_people = support::people_same_age(4_096);
    c.bench_function("sort_by_key/equal_keys", |b| {
        b.iter(|| {
            ld::sort_by_key(
                black_box(&equal_age_people),
                black_box(|person: &support::Person| person.age),
            )
        })
    });
}
