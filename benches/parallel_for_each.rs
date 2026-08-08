use criterion::Criterion;
use lowdash as ld;
use std::hint::black_box;
use std::num::NonZeroUsize;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

pub fn benchmark_parallel_for_each(c: &mut Criterion) {
    let values: Vec<i32> = (0..4_096).collect();
    let workers = NonZeroUsize::new(4).unwrap();
    let calls = Arc::new(AtomicUsize::new(0));
    c.bench_function("parallel_for_each/4_workers", |b| {
        b.iter(|| {
            let calls = Arc::clone(&calls);
            ld::parallel_for_each(black_box(&values), black_box(workers), move |_, _| {
                calls.fetch_add(1, Ordering::Relaxed);
            });
        })
    });
}
