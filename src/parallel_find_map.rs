use std::num::NonZeroUsize;
use std::panic::resume_unwind;
use std::thread;

/// Finds the lowest-index item whose callback returns `Some`.
///
/// At most `workers` scoped threads process contiguous chunks. All callback
/// results are collected before selection, so the result is independent of
/// thread completion order and callbacks may all run. Callback panics are
/// joined and resumed after all spawned threads finish.
///
/// **Time Complexity:** O(n) callback applications plus thread and result
/// overhead. Extra space is O(n) for per-item results.
///
/// # Examples
/// ```rust
/// use lowdash::parallel_find_map;
/// use std::num::NonZeroUsize;
///
/// let result = parallel_find_map(
///     &[1, 2, 3],
///     NonZeroUsize::new(2).unwrap(),
///     |value, _| (*value > 1).then_some(value * 10),
/// );
/// assert_eq!(result, Some(20));
/// ```
pub fn parallel_find_map<T, R, F>(collection: &[T], workers: NonZeroUsize, iteratee: F) -> Option<R>
where
    T: Sync,
    R: Send,
    F: Fn(&T, usize) -> Option<R> + Sync,
{
    if collection.is_empty() {
        return None;
    }

    let worker_count = workers.get().min(collection.len());
    if worker_count == 1 {
        let mut result = None;
        for (index, item) in collection.iter().enumerate() {
            if let Some(candidate) = iteratee(item, index) {
                result.get_or_insert(candidate);
            }
        }
        return result;
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

    for partial in partials {
        for item in partial {
            if item.is_some() {
                return item;
            }
        }
    }
    None
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
    fn returns_the_lowest_matching_input_index() {
        let result = parallel_find_map(&[0, 1, 2, 3], workers(2), |value, index| {
            if index == 1 {
                std::thread::sleep(Duration::from_millis(10));
            }
            (*value > 0).then_some(index)
        });

        assert_eq!(result, Some(1));
    }

    #[test]
    fn all_callbacks_may_run_before_selection() {
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = Arc::clone(&calls);
        let result = parallel_find_map(&[1, 2, 3, 4], workers(2), move |value, index| {
            callback_calls.fetch_add(1, Ordering::SeqCst);
            (index == 1).then_some(*value)
        });

        assert_eq!(result, Some(2));
        assert_eq!(calls.load(Ordering::SeqCst), 4);
    }

    #[test]
    fn handles_empty_input_more_workers_and_no_match() {
        let empty: [i32; 0] = [];
        assert_eq!(
            parallel_find_map(&empty, workers(4), |value, _| Some(*value)),
            None
        );
        assert_eq!(
            parallel_find_map(&[1, 2], workers(8), |_, _| None::<i32>),
            None
        );
    }

    #[test]
    fn supports_owned_custom_results() {
        #[derive(Debug, PartialEq)]
        struct Match {
            index: usize,
            text: String,
        }

        let result = parallel_find_map(&["a", "target", "later"], workers(2), |value, index| {
            (index > 0).then(|| Match {
                index,
                text: value.to_string(),
            })
        });

        assert_eq!(
            result,
            Some(Match {
                index: 1,
                text: "target".to_string(),
            })
        );
    }

    #[test]
    fn resumes_callback_panics() {
        let panic = catch_unwind(AssertUnwindSafe(|| {
            let _ = parallel_find_map(&[1, 2], workers(2), |value, _| {
                if *value == 2 {
                    panic!("parallel find map panic");
                }
                None::<i32>
            });
        }))
        .expect_err("callback panic should be resumed");

        assert_eq!(
            panic.downcast_ref::<&str>(),
            Some(&"parallel find map panic")
        );
    }
}
