use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;
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
}
