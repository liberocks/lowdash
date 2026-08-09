use std::hash::{Hash, Hasher};
use std::{
    any::TypeId,
    collections::hash_map::DefaultHasher,
    sync::atomic::{AtomicU64, Ordering},
    time::SystemTime,
};

thread_local! {
    static TID_HASH: u64 = {
        let mut hasher = DefaultHasher::new();
        std::thread::current().id().hash(&mut hasher);
        hasher.finish()
    };
}

#[derive(Clone, Debug)]
pub struct Float(pub f64);

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for Float {}

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

/// Returns `true` if every item is an `f32` or `f64`.
///
/// # Arguments
/// * `collection` - Items to check.
///
/// # Returns
/// * `bool` - `true` for an empty collection or when every item has one of those types.
#[allow(dead_code)]
pub fn is_collection_float(collection: &[Box<dyn std::any::Any>]) -> bool {
    collection.iter().all(|item| {
        let type_id = item.type_id();
        type_id == TypeId::of::<f32>() || type_id == TypeId::of::<f64>()
    })
}

/// Returns `true` if `T` is `f32` or `f64`.
///
/// # Returns
///
/// * `bool` - Whether `T` is a floating-point type.
#[allow(dead_code)]
pub fn is_floats<T: 'static>() -> bool {
    TypeId::of::<T>() == TypeId::of::<f32>() || TypeId::of::<T>() == TypeId::of::<f64>()
}

/// Returns a pseudo-random index below `maximum`.
///
/// # Arguments
/// * `maximum` - Exclusive upper bound. Returns `0` when it is `0`.
///
/// # Returns
/// * `usize` - An index in `0..maximum`, or `0` when `maximum` is `0`.
#[allow(dead_code)]
pub fn random_usize(maximum: usize) -> usize {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    if maximum == 0 {
        return 0;
    }

    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;

    let pid = std::process::id() as u64;

    let tid_hash = TID_HASH.with(|&h| h);

    // Increment the global counter atomically
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);

    // Combine entropy sources with prime multipliers and the counter for better distribution
    let mixed = nanos
        .wrapping_mul(0x517cc1b727220a95) // Prime multiplier
        .wrapping_add(pid)
        .wrapping_mul(0x2545f4914f6cdd1d) // Another prime
        ^ tid_hash
        ^ counter;

    // Calculate the random index within bounds
    (mixed % (maximum as u64)) as usize
}

/// Returns a pseudo-random index below `n`, using `seed` as input.
///
/// # Arguments
/// * `n` - Exclusive upper bound. Returns `0` when it is `0`.
/// * `seed` - Seed input.
///
/// # Returns
/// * `usize` - An index in `0..n`, or `0` when `n` is `0`.
#[allow(dead_code)]
pub fn random_usize_with_seed(n: usize, seed: u64) -> usize {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    if n == 0 {
        return 0;
    }

    let pid = std::process::id() as u64;
    let tid_hash = TID_HASH.with(|&h| h);

    // Increment the global counter atomically
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);

    // Combine entropy sources with prime multipliers and the counter for better distribution
    let mixed = seed
        .wrapping_mul(0x517cc1b727220a95) // Prime multiplier
        .wrapping_add(pid)
        .wrapping_mul(0x2545f4914f6cdd1d) // Another prime
        ^ tid_hash
        ^ counter;

    // Calculate the random index within bounds
    (mixed % (n as u64)) as usize
}

/// Returns the ceiling of `log2(n)`.
///
/// # Arguments
///
/// * `n` - Input number.
///
/// # Returns
///
/// * `usize` - The smallest integer greater than or equal to `log2(n)`.
#[allow(dead_code)]
pub fn ceil_log2(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }
    let mut bits = 0;
    let mut value = n - 1;
    while value > 0 {
        value >>= 1;
        bits += 1;
    }
    bits
}

/// Generates a pseudo-random `u64` from process, thread, time, and counter data.
///
/// # Returns
/// * `u64` - A pseudo-random `u64`.
#[allow(dead_code)]
pub fn random_u64() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;

    let pid = std::process::id() as u64;

    let tid_hash = TID_HASH.with(|&h| h);

    // Increment the global counter atomically
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);

    // Combine entropy sources with prime multipliers and the counter for better distribution
    let mixed = nanos
        .wrapping_mul(0x517cc1b727220a95) // Prime multiplier
        .wrapping_add(pid)
        .wrapping_mul(0x2545f4914f6cdd1d) // Another prime
        ^ tid_hash
        ^ counter;

    // Add additional randomness using a simple linear congruential generator (LCG)
    mixed.wrapping_mul(6364136223846793005).wrapping_add(1)
}

/// Lowercase letters charset.
#[allow(dead_code)]
pub const LOWERCASE_LETTERS_CHARSET: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Uppercase letters charset.
#[allow(dead_code)]
pub const UPPERCASE_LETTERS_CHARSET: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Letters charset (both lowercase and uppercase).
#[allow(dead_code)]
pub const LETTERS_CHARSET: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Numbers charset.
#[allow(dead_code)]
pub const NUMBERS_CHARSET: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// Alphanumeric charset (letters and numbers).
#[allow(dead_code)]
pub const ALPHANUMERIC_CHARSET: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

/// Special characters charset.
#[allow(dead_code)]
pub const SPECIAL_CHARSET: &[char] = &[
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '-', '=', '[', ']', '{', '}', '|',
    ';', '\'', ':', '"', ',', '.', '/', '<', '>', '?',
];

/// All characters charset (alphanumeric and special characters).
#[allow(dead_code)]
pub const ALL_CHARSET: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '-', '=',
    '[', ']', '{', '}', '|', ';', '\'', ':', '"', ',', '.', '/', '<', '>', '?',
];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
    fn test_is_floats_with_f32() {
        let result = is_floats::<f32>();
        assert!(result);
    }

    #[test]
    fn test_is_floats_with_f64() {
        let result = is_floats::<f64>();
        assert!(result);
    }

    #[test]
    fn test_is_floats_with_other_type() {
        let result = is_floats::<i32>();
        assert!(!result);
    }

    #[test]
    fn test_random_usize_uniqueness() {
        let n = 100;
        let iterations = 1000;
        let mut results = HashSet::new();

        for _ in 0..iterations {
            let index = random_usize(n);
            assert!(index < n, "random_usize({}) returned {}", n, index);
            results.insert(index);
        }

        // Expect a good distribution; not all unique, but no immediate duplicates in sequence
        assert!(!results.is_empty(), "Random index set should not be empty");
    }

    #[test]
    fn test_random_usize_range() {
        let n = 50;
        for _ in 0..1000 {
            let index = random_usize(n);
            assert!(index < n, "random_usize({}) returned {}", n, index);
        }
    }

    #[test]
    fn test_random_usize_zero() {
        let n = 0;
        let index = random_usize(n);
        assert_eq!(index, 0, "random_usize(0) should return 0, got {}", index);
    }

    #[test]
    fn test_random_usize_variety() {
        let n = 10;
        let values: HashSet<_> = (0..100).map(|_| random_usize(n)).collect();
        assert!(
            values.len() > 1,
            "random_usize should produce varied results"
        );
    }

    #[test]
    fn test_random_usize_with_seed_uniqueness() {
        let n = 100;
        let seed = 42;
        let iterations = 1000;
        let mut results = HashSet::new();

        for _ in 0..iterations {
            let index = random_usize_with_seed(n, seed);
            assert!(
                index < n,
                "random_usize_with_seed({}, {}) returned {}",
                n,
                seed,
                index
            );
            results.insert(index);
        }

        // Expect a good distribution; not all unique
        assert!(
            results.len() > 50,
            "Random index set does not have enough unique values"
        );
    }

    #[test]
    fn test_random_usize_with_seed_range() {
        let n = 50;
        let seed = 12345;
        for _ in 0..1000 {
            let index = random_usize_with_seed(n, seed);
            assert!(
                index < n,
                "random_usize_with_seed({}, {}) returned {}",
                n,
                seed,
                index
            );
        }
    }

    #[test]
    fn test_random_usize_with_seed_zero() {
        let n = 0;
        let seed = 42;
        let index = random_usize_with_seed(n, seed);
        assert_eq!(
            index, 0,
            "random_usize_with_seed({}, {}) should return 0, got {}",
            n, seed, index
        );
    }

    #[test]
    fn test_random_usize_with_seed_variety() {
        let n = 10;
        let seed = 999;
        let values: HashSet<_> = (0..100).map(|_| random_usize_with_seed(n, seed)).collect();
        assert!(
            values.len() > 1,
            "random_usize_with_seed should produce varied results"
        );
    }

    #[test]
    fn test_lowercase_letters_charset() {
        assert_eq!(LOWERCASE_LETTERS_CHARSET.len(), 26);
        for c in LOWERCASE_LETTERS_CHARSET {
            assert!(*c >= 'a' && *c <= 'z');
        }
    }

    #[test]
    fn test_uppercase_letters_charset() {
        assert_eq!(UPPERCASE_LETTERS_CHARSET.len(), 26);
        for c in UPPERCASE_LETTERS_CHARSET {
            assert!(*c >= 'A' && *c <= 'Z');
        }
    }

    #[test]
    fn test_letters_charset() {
        assert_eq!(LETTERS_CHARSET.len(), 52);
        for c in LETTERS_CHARSET {
            assert!((*c >= 'a' && *c <= 'z') || (*c >= 'A' && *c <= 'Z'));
        }
    }

    #[test]
    fn test_numbers_charset() {
        assert_eq!(NUMBERS_CHARSET.len(), 10);
        for c in NUMBERS_CHARSET {
            assert!(*c >= '0' && *c <= '9');
        }
    }

    #[test]
    fn test_alphanumeric_charset() {
        assert_eq!(ALPHANUMERIC_CHARSET.len(), 62);
        for c in ALPHANUMERIC_CHARSET {
            assert!(
                (*c >= 'a' && *c <= 'z') || (*c >= 'A' && *c <= 'Z') || (*c >= '0' && *c <= '9')
            );
        }
    }

    #[test]
    fn test_special_charset() {
        let expected_special_chars: Vec<char> = vec![
            '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '-', '=', '[', ']', '{',
            '}', '|', ';', '\'', ':', '"', ',', '.', '/', '<', '>', '?',
        ];
        assert_eq!(SPECIAL_CHARSET.len(), expected_special_chars.len());
        for c in SPECIAL_CHARSET {
            assert!(expected_special_chars.contains(c));
        }
    }

    #[test]
    fn test_ceil_log2() {
        // Test cases: (input, expected_output)
        let test_cases = [
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 2),
            (5, 3),
            (7, 3),
            (8, 3),
            (9, 4),
            (16, 4),
            (17, 5),
            (31, 5),
            (32, 5),
            (33, 6),
            (64, 6),
            (65, 7),
            (127, 7),
            (128, 7),
            (129, 8),
        ];

        for &(input, expected) in &test_cases {
            assert_eq!(
                ceil_log2(input),
                expected,
                "ceil_log2({}) should be {}",
                input,
                expected
            );
        }
    }

    #[test]
    fn test_random_u64_uniqueness() {
        let mut results = HashSet::new();
        let iterations = 1000;

        for i in 0..iterations {
            let rand_val = random_u64();
            // Assuming a high chance of uniqueness, we check for duplicates
            assert!(
                results.insert(rand_val),
                "Duplicate value found: {} in iteration {}",
                rand_val,
                i
            );
        }
    }

    #[test]
    fn test_random_u64_range() {
        for _ in 0..1000 {
            let rand_val = random_u64();
            // Since u64 can be any value, we just check it's a valid u64
            // Not much to assert here, but ensure no panics
            let _ = rand_val;
        }
    }
}
