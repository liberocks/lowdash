use criterion::{black_box, Criterion};
use lowdash as ld;

pub fn benchmark_find_last_index_of(c: &mut Criterion) {
    let ints: Vec<i32> = (0..4_096).collect();
    c.bench_function("find_last_index_of/int_vec/exists", |b| {
        b.iter(|| ld::find_last_index_of(black_box(&ints), black_box(|value: &i32| *value == 4_095)))
    });

    c.bench_function("find_last_index_of/int_vec/missing", |b| {
        b.iter(|| ld::find_last_index_of(black_box(&ints), black_box(|value: &i32| *value == -1)))
    });
}
