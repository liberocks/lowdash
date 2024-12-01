use std::time::SystemTime;

use crate::common;

/// Returns a slice of pseudo-randomly selected elements from the collection.
/// The elements are selected without replacement (no duplicates).
///
/// # Arguments
/// * `collection` - A slice of items
/// * `count` - Number of elements to sample
///
/// # Returns
/// * `Vec<T>` - A vector containing the sampled elements
///
/// # Examples
/// ```rust
/// use lowdash::samples;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result = samples(&numbers, 3);
/// assert_eq!(result.len(), 3);
/// assert!(result.iter().all(|x| numbers.contains(x)));
/// ```
pub fn samples<T>(collection: &[T], count: usize) -> Vec<T>
where
    T: Clone,
{
    let size = collection.len();
    let sample_size = size.min(count);

    let mut copy = collection.to_vec();
    let mut results = Vec::with_capacity(sample_size);

    for i in 0..sample_size {
        let copy_length = size - i;

        // Use multiple time sources for better entropy
        let seed1 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        let seed2 = std::time::Instant::now().elapsed().as_nanos() as u64;

        // Combine seeds
        let seed = (seed1 ^ seed2).wrapping_add(i as u64);

        let index = common::random_usize_with_seed(copy_length, seed);
        results.push(copy[index].clone());
        copy.swap(index, copy_length - 1);
        copy.truncate(copy_length - 1);
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_samples_basic() {
        let collection = vec![1, 2, 3, 4, 5];
        let result = samples(&collection, 3);
        assert_eq!(result.len(), 3);
        assert!(result.iter().all(|x| collection.contains(x)));
    }

    #[test]
    fn test_samples_empty_collection() {
        let collection: Vec<i32> = vec![];
        let result = samples(&collection, 3);
        assert!(result.is_empty());
    }

    #[test]
    fn test_samples_count_larger_than_collection() {
        let collection = vec![1, 2, 3];
        let result = samples(&collection, 5);
        assert_eq!(result.len(), 3);
        assert!(result.iter().all(|x| collection.contains(x)));
    }

    #[test]
    fn test_samples_with_structs() {
        #[derive(Debug, PartialEq, Clone)]
        struct Person {
            name: String,
            age: u32,
        }

        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
            },
            Person {
                name: "Bob".to_string(),
                age: 30,
            },
            Person {
                name: "Carol".to_string(),
                age: 35,
            },
        ];

        let result = samples(&people, 2);
        assert_eq!(result.len(), 2);
        assert!(result.iter().all(|x| people.contains(x)));
    }

    #[test]
    fn test_samples_no_duplicates() {
        let collection = vec![1, 2, 3, 4, 5];
        let result = samples(&collection, 3);
        let unique: HashSet<_> = result.iter().collect();
        assert_eq!(result.len(), unique.len());
    }

    #[test]
    fn test_samples_with_count_zero() {
        let collection = vec![1, 2, 3];
        let result = samples(&collection, 0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_samples_with_count_one() {
        let collection = vec![1, 2, 3];
        let result = samples(&collection, 1);
        assert_eq!(result.len(), 1);
        assert!(collection.contains(&result[0]));
    }

    #[test]
    fn test_samples_full_collection() {
        let collection = vec![1, 2, 3];
        let result = samples(&collection, 3);
        assert_eq!(result.len(), 3);
        let mut sorted_result = result.clone();
        sorted_result.sort();
        assert_eq!(sorted_result, collection);
    }
    #[test]
    fn test_samples_distribution() {
        let collection = vec![1, 2, 3, 4, 5];
        let mut seen_results = Vec::new();

        // Take multiple samples and store them
        for _ in 0..100 {
            let result = samples(&collection, 2);
            seen_results.push(result);

            // Add varying delays between samples
            for _ in 0..((seen_results.len() % 20) + 1) {
                std::hint::spin_loop();
            }
        }

        // Basic distribution checks:

        // 1. All samples should be the correct size
        assert!(seen_results.iter().all(|r| r.len() == 2));

        // 2. All values should be from the original collection
        assert!(seen_results
            .iter()
            .all(|result| result.iter().all(|x| collection.contains(x))));

        // 3. No sample should have duplicates
        assert!(seen_results.iter().all(|result| {
            let unique: HashSet<_> = result.iter().collect();
            result.len() == unique.len()
        }));

        // 4. We should see at least some variation in the samples
        let first_sample = &seen_results[0];
        assert!(seen_results.iter().any(|result| result != first_sample));
    }
}
