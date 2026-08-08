/// Counts ordered selections of `k` items from `n` items without repetition.
///
/// Returns `Some(0)` when `k` is greater than `n`, and `None` when the result
/// cannot be represented by `u128`.
///
/// # Complexity
///
/// Runs in `O(k)` time until overflow and uses `O(1)` space.
///
/// # Examples
///
/// ```rust
/// use lowdash::permutation_count;
///
/// assert_eq!(permutation_count(5, 2), Some(20));
/// assert_eq!(permutation_count(5, 0), Some(1));
/// assert_eq!(permutation_count(2, 3), Some(0));
/// ```
pub fn permutation_count(n: u64, k: u64) -> Option<u128> {
    if k > n {
        return Some(0);
    }

    let mut result = 1_u128;
    for offset in 0..k {
        result = result.checked_mul((n - offset) as u128)?;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation_count_zero_selection() {
        assert_eq!(permutation_count(0, 0), Some(1));
        assert_eq!(permutation_count(5, 0), Some(1));
    }

    #[test]
    fn test_permutation_count_computes_ordered_selection_count() {
        assert_eq!(permutation_count(5, 2), Some(20));
        assert_eq!(permutation_count(4, 4), Some(24));
    }

    #[test]
    fn test_permutation_count_k_greater_than_n() {
        assert_eq!(permutation_count(2, 3), Some(0));
        assert_eq!(permutation_count(0, 1), Some(0));
    }

    #[test]
    fn test_permutation_count_u64_boundary() {
        assert_eq!(permutation_count(u64::MAX, 1), Some(u64::MAX as u128));
    }

    #[test]
    fn test_permutation_count_overflow() {
        assert_eq!(permutation_count(35, 35), None);
        assert_eq!(permutation_count(u64::MAX, 3), None);
    }
}
