use criterion::black_box;
use criterion::Criterion;
use lowdash as ld;
use std::num::NonZeroUsize;

pub fn benchmark_parallel_try_map(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).collect();
    let workers = NonZeroUsize::new(4).unwrap();
    c.bench_function("parallel_try_map/success", |b| {
        b.iter(|| {
            ld::parallel_try_map(
                black_box(&values),
                black_box(workers),
                black_box(|value: &i32, index| Ok::<i32, ()>(*value + index as i32)),
            )
        })
    });

    let one_worker = NonZeroUsize::new(1).unwrap();
    c.bench_function("parallel_try_map/one_worker", |b| {
        b.iter(|| {
            ld::parallel_try_map(
                black_box(&values),
                black_box(one_worker),
                black_box(|value: &i32, index| Ok::<i32, ()>(*value + index as i32)),
            )
        })
    });
}
