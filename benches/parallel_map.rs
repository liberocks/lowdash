use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;
use std::num::NonZeroUsize;

pub fn benchmark_parallel_map(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).collect();
    let workers = NonZeroUsize::new(4).unwrap();
    c.bench_function("parallel_map/4_workers", |b| {
        b.iter(|| {
            ld::parallel_map(
                black_box(&values),
                black_box(workers),
                black_box(|value: &i32, index| *value + index as i32),
            )
        })
    });

    let small: Vec<i32> = (0..128).collect();
    c.bench_function("parallel_map/worker_per_item", |b| {
        b.iter(|| {
            ld::parallel_map(
                black_box(&small),
                black_box(NonZeroUsize::new(128).unwrap()),
                black_box(|value: &i32, _| *value * 2),
            )
        })
    });
}
