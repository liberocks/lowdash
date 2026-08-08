/// Returns the least common multiple of all values.
///
/// An empty collection returns `Some(1)`, the multiplicative identity. If any
/// value is zero, the result is `Some(0)`. Returns `None` when the nonzero
/// result cannot be represented by `u64`.
///
/// The calculation divides by the greatest common divisor before multiplying,
/// minimizing the chance of checked overflow.
///
/// # Complexity
///
/// Runs in `O(values.len() * log(max(values)))` time and `O(1)` space.
///
/// # Examples
///
/// ```rust
/// use lowdash::least_common_multiple;
///
/// assert_eq!(least_common_multiple(&[4, 6, 8]), Some(24));
/// assert_eq!(least_common_multiple(&[]), Some(1));
/// assert_eq!(least_common_multiple(&[0, 12]), Some(0));
/// ```
pub fn least_common_multiple(values: &[u64]) -> Option<u64> {
    if values.contains(&0) {
        return Some(0);
    }

    let mut result = 1_u64;
    for &value in values {
        let divisor = crate::greatest_common_divisor(&[result, value]);
        result = (result / divisor).checked_mul(value)?;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_common_multiple_empty() {
        assert_eq!(least_common_multiple(&[]), Some(1));
    }

    #[test]
    fn test_least_common_multiple_zero_values() {
        assert_eq!(least_common_multiple(&[0]), Some(0));
        assert_eq!(least_common_multiple(&[0, 12]), Some(0));
        assert_eq!(least_common_multiple(&[u64::MAX, u64::MAX - 1, 0]), Some(0));
    }

    #[test]
    fn test_least_common_multiple_multiple_values() {
        assert_eq!(least_common_multiple(&[4, 6, 8]), Some(24));
        assert_eq!(least_common_multiple(&[21, 6, 14]), Some(42));
    }

    #[test]
    fn test_least_common_multiple_boundary() {
        assert_eq!(least_common_multiple(&[u64::MAX]), Some(u64::MAX));
        assert_eq!(least_common_multiple(&[1, u64::MAX]), Some(u64::MAX));
    }

    #[test]
    fn test_least_common_multiple_overflow() {
        assert_eq!(least_common_multiple(&[u64::MAX, u64::MAX - 1]), None);
    }
}
