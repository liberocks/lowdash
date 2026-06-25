use crate::support;
use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_splice(c: &mut Criterion) {
    let ints = support::int_vec(4_096);
    let replacement = vec![999; 128];
    c.bench_function("splice/int_vec/remove", |b| {
        b.iter(|| ld::splice(black_box(&ints), black_box(0), black_box(&[] as &[i32])))
    });

    c.bench_function("splice/int_vec/replace", |b| {
        b.iter(|| ld::splice(black_box(&ints), black_box(256), black_box(&replacement)))
    });
}
