use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;
use std::num::NonZeroUsize;

pub fn benchmark_parallel_find_map(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).collect();
    let workers = NonZeroUsize::new(4).unwrap();
    c.bench_function("parallel_find_map/late_match", |b| {
        b.iter(|| {
            ld::parallel_find_map(
                black_box(&values),
                black_box(workers),
                black_box(|value: &i32, _| (*value == 4_000).then_some(*value)),
            )
        })
    });

    let one_worker = NonZeroUsize::new(1).unwrap();
    c.bench_function("parallel_find_map/one_worker", |b| {
        b.iter(|| {
            ld::parallel_find_map(
                black_box(&values),
                black_box(one_worker),
                black_box(|value: &i32, _| (*value == 4_000).then_some(*value)),
            )
        })
    });
}
