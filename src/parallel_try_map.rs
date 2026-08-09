use std::num::NonZeroUsize;
use std::panic::resume_unwind;
use std::thread;

/// Maps a collection concurrently, returning the first error by input order.
///
/// At most `workers` scoped threads process contiguous chunks and results keep
/// global input order. Callbacks in other chunks are not cancelled when an
/// error occurs, so later callbacks may already have run. Callback panics are
/// joined and resumed after all spawned threads finish.
///
/// **Time Complexity:** O(n) callback applications plus thread and result
/// overhead. Extra space is O(n).
///
/// # Examples
/// ```rust
/// use lowdash::parallel_try_map;
/// use std::num::NonZeroUsize;
///
/// let result = parallel_try_map(
///     &[1, 2, 3],
///     NonZeroUsize::new(2).unwrap(),
///     |value, index| if index == 1 { Err("bad value") } else { Ok(value * 2) },
/// );
/// assert_eq!(result, Err("bad value"));
/// ```
pub fn parallel_try_map<T, R, E, F>(
    collection: &[T],
    workers: NonZeroUsize,
    iteratee: F,
) -> Result<Vec<R>, E>
where
    T: Sync,
    R: Send,
    E: Send,
    F: Fn(&T, usize) -> Result<R, E> + Sync,
{
    if collection.is_empty() {
        return Ok(Vec::new());
    }

    let worker_count = workers.get().min(collection.len());
    if worker_count == 1 {
        let mut result = Vec::with_capacity(collection.len());
        let mut first_error = None;
        for (index, item) in collection.iter().enumerate() {
            match iteratee(item, index) {
                Ok(value) if first_error.is_none() => result.push(value),
                Ok(_) => {}
                Err(error) if first_error.is_none() => first_error = Some(error),
                Err(_) => {}
            }
        }
        return first_error.map_or(Ok(result), Err);
    }

    let chunk_size =
        collection.len() / worker_count + usize::from(collection.len() % worker_count != 0);
    let (partials, panic_payload) = thread::scope(|scope| {
        let mut handles = Vec::with_capacity(worker_count);
        let iteratee = &iteratee;

        for (chunk_index, chunk) in collection.chunks(chunk_size).enumerate() {
            handles.push(scope.spawn(move || {
                chunk
                    .iter()
                    .enumerate()
                    .map(|(offset, item)| iteratee(item, chunk_index * chunk_size + offset))
                    .collect::<Vec<_>>()
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

    let mut result = Vec::with_capacity(collection.len());
    for partial in partials {
        for item in partial {
            result.push(item?);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use std::time::Duration;

    fn workers(count: usize) -> NonZeroUsize {
        NonZeroUsize::new(count).unwrap()
    }

    #[test]
    fn preserves_order_and_passes_global_indices_on_success() {
        let result = parallel_try_map(&[10, 20, 30], workers(2), |value, index| {
            Ok::<_, ()>(*value + index as i32)
        });

        assert_eq!(result, Ok(vec![10, 21, 32]));
    }

    #[test]
    fn returns_the_first_error_by_input_order() {
        let result = parallel_try_map(&[0, 1, 2, 3], workers(2), |value, index| {
            if index == 1 {
                std::thread::sleep(Duration::from_millis(10));
                Err(*value)
            } else if index == 2 {
                Err(*value)
            } else {
                Ok(*value)
            }
        });

        assert_eq!(result, Err(1));
    }

    #[test]
    fn later_callbacks_are_not_cancelled_after_an_error() {
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = Arc::clone(&calls);
        let result = parallel_try_map(&[1, 2, 3, 4], workers(2), move |value, index| {
            callback_calls.fetch_add(1, Ordering::SeqCst);
            if index == 1 {
                Err(*value)
            } else {
                Ok(*value)
            }
        });

        assert_eq!(result, Err(2));
        assert_eq!(calls.load(Ordering::SeqCst), 4);
    }

    #[test]
    fn handles_empty_input_and_more_workers_than_items() {
        let empty: [i32; 0] = [];
        assert_eq!(
            parallel_try_map(&empty, workers(4), |value, _| Ok::<_, ()>(*value)),
            Ok(Vec::<i32>::new())
        );

        assert_eq!(
            parallel_try_map(&[1, 2], workers(8), |value, _| Ok::<_, ()>(*value * 2)),
            Ok(vec![2, 4])
        );
    }

    #[test]
    fn handles_one_worker_and_runs_callbacks_after_errors() {
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = Arc::clone(&calls);
        let result = parallel_try_map(&[1, 2, 3, 4], workers(1), move |value, index| {
            callback_calls.fetch_add(1, Ordering::SeqCst);
            if index == 1 || index == 3 {
                Err(*value)
            } else {
                Ok(*value)
            }
        });

        assert_eq!(result, Err(2));
        assert_eq!(calls.load(Ordering::SeqCst), 4);
    }

    #[test]
    fn supports_owned_custom_results_and_errors() {
        #[derive(Debug, PartialEq)]
        struct Label(String);

        let result = parallel_try_map(&["a", "b"], workers(2), |value, index| {
            Ok::<_, &'static str>(Label(format!("{}{}", value, index)))
        });

        assert_eq!(
            result,
            Ok(vec![Label("a0".to_string()), Label("b1".to_string())])
        );
    }

    #[test]
    fn resumes_callback_panics() {
        let panic = catch_unwind(AssertUnwindSafe(|| {
            let _ = parallel_try_map(&[1, 2], workers(2), |value, _| {
                if *value == 2 {
                    panic!("parallel try map panic");
                }
                Ok::<_, ()>(*value)
            });
        }))
        .expect_err("callback panic should be resumed");

        assert_eq!(
            panic.downcast_ref::<&str>(),
            Some(&"parallel try map panic")
        );
    }
}
