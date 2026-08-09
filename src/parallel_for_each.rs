use std::num::NonZeroUsize;
use std::panic::resume_unwind;
use std::thread;

/// Applies a callback concurrently to contiguous chunks without collecting output.
///
/// At most `workers` scoped threads are used. The callback receives each
/// item's global input index, but callback execution order is unspecified. All
/// spawned threads are joined before a callback panic is resumed.
///
/// **Time Complexity:** O(n) callback applications plus thread overhead. No
/// output collection is allocated.
///
/// # Examples
/// ```rust
/// use lowdash::parallel_for_each;
/// use std::num::NonZeroUsize;
/// use std::sync::{Arc, Mutex};
///
/// let seen = Arc::new(Mutex::new(Vec::new()));
/// let output = Arc::clone(&seen);
/// parallel_for_each(&[10, 20], NonZeroUsize::new(2).unwrap(), move |value, index| {
///     output.lock().unwrap().push((*value, index));
/// });
/// assert_eq!(seen.lock().unwrap().len(), 2);
/// ```
pub fn parallel_for_each<T, F>(collection: &[T], workers: NonZeroUsize, iteratee: F)
where
    T: Sync,
    F: Fn(&T, usize) + Sync,
{
    if collection.is_empty() {
        return;
    }

    let worker_count = workers.get().min(collection.len());
    if worker_count == 1 || collection.len() <= worker_count.saturating_mul(2) {
        for (index, item) in collection.iter().enumerate() {
            iteratee(item, index);
        }
        return;
    }

    let chunk_size =
        collection.len() / worker_count + usize::from(collection.len() % worker_count != 0);
    let panic_payload = thread::scope(|scope| {
        let mut handles = Vec::with_capacity(worker_count);
        let iteratee = &iteratee;

        for (chunk_index, chunk) in collection.chunks(chunk_size).enumerate() {
            handles.push(scope.spawn(move || {
                for (offset, item) in chunk.iter().enumerate() {
                    iteratee(item, chunk_index * chunk_size + offset);
                }
            }));
        }

        let mut panic_payload = None;
        for handle in handles {
            if let Err(payload) = handle.join() {
                if panic_payload.is_none() {
                    panic_payload = Some(payload);
                }
            }
        }
        panic_payload
    });

    if let Some(payload) = panic_payload {
        resume_unwind(payload);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    };

    fn workers(count: usize) -> NonZeroUsize {
        NonZeroUsize::new(count).unwrap()
    }

    #[test]
    fn processes_every_item_and_passes_global_indices() {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let callback_seen = Arc::clone(&seen);
        parallel_for_each(&[10, 20, 30, 40], workers(2), move |value, index| {
            callback_seen.lock().unwrap().push((*value, index));
        });

        let mut seen = Arc::try_unwrap(seen).unwrap().into_inner().unwrap();
        seen.sort_unstable_by_key(|(_, index)| *index);
        assert_eq!(seen, vec![(10, 0), (20, 1), (30, 2), (40, 3)]);
    }

    #[test]
    fn handles_empty_input_and_more_workers_than_items() {
        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = Arc::clone(&calls);
        let empty: [i32; 0] = [];
        parallel_for_each(&empty, workers(4), move |_, _| {
            callback_calls.fetch_add(1, Ordering::SeqCst);
        });
        assert_eq!(calls.load(Ordering::SeqCst), 0);

        let calls = Arc::new(AtomicUsize::new(0));
        let callback_calls = Arc::clone(&calls);
        parallel_for_each(&[1, 2], workers(8), move |_, _| {
            callback_calls.fetch_add(1, Ordering::SeqCst);
        });
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn supports_custom_owned_input_values() {
        #[derive(Debug)]
        struct Item(String);

        let items = [Item("a".to_string()), Item("b".to_string())];
        let seen = Arc::new(Mutex::new(Vec::new()));
        let callback_seen = Arc::clone(&seen);
        parallel_for_each(&items, workers(2), move |item, index| {
            callback_seen.lock().unwrap().push((item.0.clone(), index));
        });

        let mut seen = seen.lock().unwrap().clone();
        seen.sort_unstable_by_key(|(_, index)| *index);
        assert_eq!(seen, vec![("a".to_string(), 0), ("b".to_string(), 1)]);
    }

    #[test]
    fn resumes_callback_panics_after_joining_threads() {
        let completed = Arc::new(AtomicUsize::new(0));
        let callback_completed = Arc::clone(&completed);
        let panic = catch_unwind(AssertUnwindSafe(|| {
            parallel_for_each(&[1, 2, 3, 4], workers(2), move |value, _| {
                if *value == 2 {
                    panic!("parallel for each panic");
                }
                callback_completed.fetch_add(1, Ordering::SeqCst);
            });
        }))
        .expect_err("callback panic should be resumed");

        assert_eq!(
            panic.downcast_ref::<&str>(),
            Some(&"parallel for each panic")
        );
        assert!(completed.load(Ordering::SeqCst) >= 1);
    }
}
