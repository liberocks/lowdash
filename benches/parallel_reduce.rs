use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;
use std::num::NonZeroUsize;

pub fn benchmark_parallel_reduce(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).collect();
    let workers = NonZeroUsize::new(4).unwrap();
    c.bench_function("parallel_reduce/sum", |b| {
        b.iter(|| {
            ld::parallel_reduce(
                black_box(&values),
                black_box(workers),
                black_box(|| 0_i32),
                black_box(|sum: i32, value: &i32, index| sum + *value + index as i32),
                black_box(|left: i32, right: i32| left + right),
            )
        })
    });

    let one_worker = NonZeroUsize::new(1).unwrap();
    c.bench_function("parallel_reduce/one_worker", |b| {
        b.iter(|| {
            ld::parallel_reduce(
                black_box(&values),
                black_box(one_worker),
                black_box(|| 0_i32),
                black_box(|sum: i32, value: &i32, index| sum + *value + index as i32),
                black_box(|left: i32, right: i32| left + right),
            )
        })
    });
}
