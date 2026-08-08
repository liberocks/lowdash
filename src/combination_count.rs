/// Counts unordered selections of `k` items from `n` items without repetition.
///
/// Returns `Some(0)` when `k` is greater than `n`, and `None` when the result
/// cannot be represented by `u128`.
///
/// Common factors are cancelled before each checked multiplication, so the
/// calculation does not construct a factorial or an avoidable intermediate
/// product.
///
/// # Complexity
///
/// Runs in `O(min(k, n - k) * log(n))` time and `O(1)` space.
///
/// # Examples
///
/// ```rust
/// use lowdash::combination_count;
///
/// assert_eq!(combination_count(5, 2), Some(10));
/// assert_eq!(combination_count(5, 0), Some(1));
/// assert_eq!(combination_count(2, 3), Some(0));
/// ```
pub fn combination_count(n: u64, k: u64) -> Option<u128> {
    if k > n {
        return Some(0);
    }

    let k = k.min(n - k);
    let mut result = 1_u128;

    for i in 1..=k {
        let mut numerator = (n - i + 1) as u128;
        let mut denominator = i as u128;

        let common = greatest_common_divisor(numerator, denominator);
        numerator /= common;
        denominator /= common;

        let common = greatest_common_divisor(result, denominator);
        result = result.checked_div(common)?;
        denominator /= common;

        if denominator != 1 {
            return None;
        }

        result = result.checked_mul(numerator)?;
    }

    Some(result)
}

fn greatest_common_divisor(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_count_zero_choose_zero() {
        assert_eq!(combination_count(0, 0), Some(1));
    }

    #[test]
    fn test_combination_count_computes_selection_count() {
        assert_eq!(combination_count(5, 2), Some(10));
        assert_eq!(combination_count(52, 5), Some(2_598_960));
    }

    #[test]
    fn test_combination_count_uses_symmetry_without_factorial_overflow() {
        assert!(combination_count(128, 64).is_some());
    }

    #[test]
    fn test_combination_count_k_greater_than_n() {
        assert_eq!(combination_count(2, 3), Some(0));
    }

    #[test]
    fn test_combination_count_u64_boundary() {
        assert_eq!(combination_count(u64::MAX, 1), Some(u64::MAX as u128));
    }

    #[test]
    fn test_combination_count_overflow() {
        assert_eq!(combination_count(u64::MAX, 3), None);
    }
}
