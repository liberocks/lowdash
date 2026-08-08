use std::num::NonZeroUsize;
use std::panic::resume_unwind;
use std::thread;

/// Reduces contiguous chunks concurrently and combines them in chunk order.
///
/// `initial` should provide the identity value for both `fold` and `combine`,
/// and `combine` should be associative for results independent of chunking.
/// Floating-point reductions can still differ slightly because grouping changes
/// rounding. The combiner runs sequentially in the caller thread. Callback
/// panics are joined and resumed after all spawned threads finish.
///
/// An empty collection calls `initial` exactly once. Non-empty collections call
/// it once per chunk, with at most `workers` chunks.
///
/// **Time Complexity:** O(n) callback applications plus O(c) combination work,
/// where `c` is the number of chunks. Extra space is O(c) for partial results.
///
/// # Examples
/// ```rust
/// use lowdash::parallel_reduce;
/// use std::num::NonZeroUsize;
///
/// let result = parallel_reduce(
///     &[1, 2, 3, 4],
///     NonZeroUsize::new(2).unwrap(),
///     || 0,
///     |sum, value, _| sum + value,
///     |left, right| left + right,
/// );
/// assert_eq!(result, 10);
/// ```
pub fn parallel_reduce<T, R, FI, FF, FC>(
    collection: &[T],
    workers: NonZeroUsize,
    initial: FI,
    fold: FF,
    mut combine: FC,
) -> R
where
    T: Sync,
    R: Send,
    FI: Fn() -> R + Sync,
    FF: Fn(R, &T, usize) -> R + Sync,
    FC: FnMut(R, R) -> R,
{
    if collection.is_empty() {
        return initial();
    }

    let worker_count = workers.get().min(collection.len());
    let chunk_size =
        collection.len() / worker_count + usize::from(collection.len() % worker_count != 0);
    let (partials, panic_payload) = thread::scope(|scope| {
        let mut handles = Vec::with_capacity(worker_count);
        let initial = &initial;
        let fold = &fold;

        for (chunk_index, chunk) in collection.chunks(chunk_size).enumerate() {
            handles.push(scope.spawn(move || {
                let mut result = initial();
                for (offset, item) in chunk.iter().enumerate() {
                    result = fold(result, item, chunk_index * chunk_size + offset);
                }
                result
            }));
        }

        let mut partials = Vec::with_capacity(handles.len());
        let mut panic_payload = None;
        for handle in handles {
            match handle.join() {
                Ok(partial) => partials.push(partial),
                Err(payload) => {
                    if panic_payload.is_none() {
                        panic_payload = Some(payload);
                    }
                }
            }
        }

        (partials, panic_payload)
    });

    if let Some(payload) = panic_payload {
        resume_unwind(payload);
    }

    let mut partials = partials.into_iter();
    let mut result = partials
        .next()
        .expect("a non-empty collection has at least one partial result");
    for partial in partials {
        result = combine(result, partial);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    fn workers(count: usize) -> NonZeroUsize {
        NonZeroUsize::new(count).unwrap()
    }

    #[test]
    fn folds_items_with_global_indices_and_chunk_order() {
        let result = parallel_reduce(
            &[1, 2, 3, 4],
            workers(2),
            || 0,
            |sum, value, index| sum + *value + index as i32,
            |left, right| left + right,
        );

        assert_eq!(result, 16);
    }

    #[test]
    fn calls_initial_once_for_empty_input() {
        let calls = Arc::new(AtomicUsize::new(0));
        let initial_calls = Arc::clone(&calls);
        let result = parallel_reduce(
            &[] as &[i32],
            workers(8),
            move || {
                initial_calls.fetch_add(1, Ordering::SeqCst);
                42
            },
            |sum, value, _| sum + *value,
            |left, right| left + right,
        );

        assert_eq!(result, 42);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn handles_more_workers_than_items() {
        let result = parallel_reduce(
            &[1, 2],
            workers(8),
            || 0,
            |sum, value, _| sum + *value,
            |left, right| left + right,
        );

        assert_eq!(result, 3);
    }

    #[test]
    fn supports_owned_custom_accumulators() {
        let result = parallel_reduce(
            &["a", "b", "c"],
            workers(2),
            String::new,
            |mut output, value, index| {
                output.push_str(value);
                output.push_str(&index.to_string());
                output
            },
            |mut left, right| {
                left.push_str(&right);
                left
            },
        );

        assert_eq!(result, "a0b1c2");
    }

    #[test]
    fn resumes_fold_panics_after_joining_threads() {
        let panic = catch_unwind(AssertUnwindSafe(|| {
            let _ = parallel_reduce(
                &[1, 2, 3],
                workers(2),
                || 0,
                |sum, value, _| {
                    if *value == 2 {
                        panic!("parallel reduce panic");
                    }
                    sum + *value
                },
                |left, right| left + right,
            );
        }))
        .expect_err("fold panic should be resumed");

        assert_eq!(panic.downcast_ref::<&str>(), Some(&"parallel reduce panic"));
    }
}
