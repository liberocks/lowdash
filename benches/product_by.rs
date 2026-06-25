use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_product_by(c: &mut Criterion) {
    let people_increasing = support::people(4_096);
    c.bench_function("product_by/people/increasing", |b| {
        b.iter(|| {
            ld::product_by(
                black_box(&people_increasing),
                black_box(|person: &support::Person| 1.0 + (person.age % 3) as f64 / 100.0),
            )
        })
    });

    let people_same = support::people_same_age(4_096);
    c.bench_function("product_by/people/equal", |b| {
        b.iter(|| {
            ld::product_by(
                black_box(&people_same),
                black_box(|person: &support::Person| 1.0 + (person.age % 3) as f64 / 100.0),
            )
        })
    });

    let ints = support::int_vec(4_096);
    c.bench_function("product_by/int_vec", |b| {
        b.iter(|| {
            ld::product_by(
                black_box(&ints),
                black_box(|x: &i32| 1.0 + (*x).unsigned_abs() as f64 / 100.0),
            )
        })
    });
}
