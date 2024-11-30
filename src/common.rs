use std::any::TypeId;

/// Determines if the collection contains floating-point numbers (`f32` or `f64`).
///
/// # Arguments
/// * `collection` - A slice of items.
///
/// # Returns
/// * `true` if all elements in the collection are of type `f32` or `f64`, otherwise `false`.
pub fn is_collection_float(collection: &[Box<dyn std::any::Any>]) -> bool {
    collection.iter().all(|item| {
        let type_id = item.type_id();
        type_id == TypeId::of::<f32>() || type_id == TypeId::of::<f64>()
    })
}

/// Returns a pseudo-random index from the collection.
///
/// # Arguments
/// * `n` - The upper bound of the random index (exclusive).
///
/// # Returns
/// * `usize` - A pseudo-random index from 0 to n-1.
pub fn random_index(n: usize) -> usize {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    nanos as usize % n
}

/// Returns a pseudo-random index from the collection using a seed.
///
/// # Arguments
/// * `n` - The upper bound of the random index (exclusive).
/// * `seed` - The seed value for the random number generator.
///
/// # Returns
/// * `usize` - A pseudo-random index from 0 to n-1.
pub fn random_index_with_seed(n: usize, seed: u64) -> usize {
    if n == 0 {
        return 0;
    }

    // Combine multiple entropy sources
    let pid = std::process::id() as u64;
    let tid_hash = format!("{:?}", std::thread::current().id())
        .bytes()
        .fold(0u64, |acc, b| acc.wrapping_add(b as u64));

    // Mix entropy sources with seed using XOR
    let mixed = seed
        .wrapping_mul(0x517cc1b727220a95) // Prime multiplier
        .wrapping_add(pid)
        .wrapping_mul(0x2545f4914f6cdd1d) // Another prime
        ^ tid_hash;

    (mixed % (n as u64)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_collection_float_with_float() {
        let collection: Vec<Box<dyn std::any::Any>> =
            vec![Box::new(1.0f64), Box::new(2.0f64), Box::new(3.0f64)];
        let result = is_collection_float(&collection);
        assert!(result);
    }

    #[test]
    fn test_is_collection_float_with_integer() {
        let collection: Vec<Box<dyn std::any::Any>> = vec![Box::new(1), Box::new(2), Box::new(3)];
        let result = is_collection_float(&collection);
        assert!(!result);
    }

    #[test]
    fn test_is_collection_float_with_mixed_types() {
        let collection: Vec<Box<dyn std::any::Any>> =
            vec![Box::new(1.0f64), Box::new(2), Box::new(3.0f64)];
        let result = is_collection_float(&collection);
        assert!(!result);
    }

    #[test]
    fn test_random_index() {
        let n = 10;
        let result = random_index(n);
        assert!(result < n);
    }

    #[test]
    fn test_random_index_with_seed() {
        let n = 10;
        let seed = 42;
        let result = random_index_with_seed(n, seed);
        assert!(result < n);
    }
}
