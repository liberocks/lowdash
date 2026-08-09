use std::num::NonZeroUsize;
use std::panic::resume_unwind;
use std::thread;

/// Maps a collection concurrently over contiguous chunks.
///
/// At most `workers` scoped threads are used. Results retain input order and
/// the callback receives each item's global input index. If a callback panics,
/// all spawned threads are joined and the panic payload is resumed.
///
/// **Time Complexity:** O(n) callback applications plus thread and result
/// overhead. Extra space is O(n) for the returned results.
///
/// # Examples
/// ```rust
/// use lowdash::parallel_map;
/// use std::num::NonZeroUsize;
///
/// let workers = NonZeroUsize::new(2).unwrap();
/// let result = parallel_map(&[10, 20, 30], workers, |value, index| {
///     value + index as i32
/// });
/// assert_eq!(result, vec![10, 21, 32]);
/// ```
pub fn parallel_map<T, R, F>(collection: &[T], workers: NonZeroUsize, iteratee: F) -> Vec<R>
where
    T: Sync,
    R: Send,
    F: Fn(&T, usize) -> R + Sync,
{
    if collection.is_empty() {
        return Vec::new();
    }

    let worker_count = workers.get().min(collection.len());
    if worker_count == 1 || collection.len() <= worker_count.saturating_mul(2) {
        return collection
            .iter()
            .enumerate()
            .map(|(index, item)| iteratee(item, index))
            .collect();
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
        result.extend(partial);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    fn workers(count: usize) -> NonZeroUsize {
        NonZeroUsize::new(count).unwrap()
    }

    #[test]
    fn preserves_order_and_passes_global_indices() {
        let result = parallel_map(&[10, 20, 30, 40, 50], workers(2), |value, index| {
            (*value, index)
        });

        assert_eq!(result, vec![(10, 0), (20, 1), (30, 2), (40, 3), (50, 4)]);
    }

    #[test]
    fn handles_empty_input() {
        let empty: [i32; 0] = [];

        assert_eq!(
            parallel_map(&empty, workers(4), |value, _| *value),
            Vec::new()
        );
    }

    #[test]
    fn handles_more_workers_than_items() {
        let result = parallel_map(&[1, 2], workers(8), |value, index| *value + index as i32);

        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn supports_owned_custom_results() {
        #[derive(Debug, PartialEq)]
        struct Label {
            index: usize,
            text: String,
        }

        let result = parallel_map(&["a", "b"], workers(2), |value, index| Label {
            index,
            text: format!("{}{}", value, index),
        });

        assert_eq!(
            result,
            vec![
                Label {
                    index: 0,
                    text: "a0".to_string(),
                },
                Label {
                    index: 1,
                    text: "b1".to_string(),
                },
            ]
        );
    }

    #[test]
    fn resumes_callback_panics() {
        let panic = catch_unwind(AssertUnwindSafe(|| {
            parallel_map(&[1, 2, 3, 4, 5], workers(2), |value, _| {
                if *value == 2 || *value == 4 {
                    panic!("parallel map panic");
                }
                *value
            });
        }))
        .expect_err("callback panic should be resumed");

        assert_eq!(panic.downcast_ref::<&str>(), Some(&"parallel map panic"));
    }
}
