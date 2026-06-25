use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_find_or_else(c: &mut Criterion) {
    let fallback = -1;
    let ints: Vec<i32> = (0..4_096).collect();
    c.bench_function("find_or_else/int_vec/exists", |b| {
        b.iter(|| {
            ld::find_or_else(
                black_box(&ints), black_box(&fallback),
                black_box(|value: &i32| *value == 4_095),
            )
        })
    });

    c.bench_function("find_or_else/int_vec/missing", |b| {
        b.iter(|| {
            ld::find_or_else(
                black_box(&ints), black_box(&fallback),
                black_box(|value: &i32| *value == -1),
            )
        })
    });
}
