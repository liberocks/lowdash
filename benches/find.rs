use criterion::{black_box, Criterion};
use lowdash::common::random_usize;
use lowdash::find;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

pub fn benchmark_find_i32(c: &mut Criterion) {
    let collection: Vec<i32> = (0..1000).collect();
    let predicate = |x: &i32| *x == 500;
    c.bench_function("find i32", |b| {
        b.iter(|| find(black_box(&collection), black_box(&predicate)))
    });
}

pub fn benchmark_find_float(c: &mut Criterion) {
    let collection: Vec<f64> = (0..1000).map(|x| x as f64).collect();
    let predicate = |x: &f64| *x == 500.0;
    c.bench_function("find f64", |b| {
        b.iter(|| find(black_box(&collection), black_box(&predicate)))
    });
}

pub fn benchmark_find_person(c: &mut Criterion) {
    let people: Vec<Person> = (0..1000)
        .map(|_| Person {
            name: format!("Person{}", random_usize(1000)),
            age: random_usize(100) as u32,
        })
        .collect();
    let predicate = |p: &Person| p.age > 30;
    c.bench_function("find Person", |b| {
        b.iter(|| find(black_box(&people), black_box(&predicate)))
    });
}
