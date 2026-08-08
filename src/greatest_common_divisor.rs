/// Returns the greatest common divisor of all values.
///
/// The empty collection has a greatest common divisor of `0`. Zero values
/// follow the standard Euclidean semantics: the greatest common divisor of
/// zero and a nonzero value is that nonzero value, while all-zero input returns
/// zero.
///
/// # Complexity
///
/// Runs in `O(values.len() * log(max(values)))` time and `O(1)` space.
///
/// # Examples
///
/// ```rust
/// use lowdash::greatest_common_divisor;
///
/// assert_eq!(greatest_common_divisor(&[48, 18, 30]), 6);
/// assert_eq!(greatest_common_divisor(&[0, 18]), 18);
/// assert_eq!(greatest_common_divisor(&[]), 0);
/// ```
pub fn greatest_common_divisor(values: &[u64]) -> u64 {
    let mut result = 0;

    for &value in values {
        result = greatest_common_divisor_pair(result, value);
        if result == 1 {
            return 1;
        }
    }

    result
}

fn greatest_common_divisor_pair(mut left: u64, mut right: u64) -> u64 {
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
    fn test_greatest_common_divisor_empty() {
        assert_eq!(greatest_common_divisor(&[]), 0);
    }

    #[test]
    fn test_greatest_common_divisor_zero_values() {
        assert_eq!(greatest_common_divisor(&[0]), 0);
        assert_eq!(greatest_common_divisor(&[0, 0]), 0);
        assert_eq!(greatest_common_divisor(&[0, 18, 0]), 18);
    }

    #[test]
    fn test_greatest_common_divisor_multiple_values() {
        assert_eq!(greatest_common_divisor(&[48, 18, 30]), 6);
        assert_eq!(greatest_common_divisor(&[24, 36, 60]), 12);
    }

    #[test]
    fn test_greatest_common_divisor_single_boundary_value() {
        assert_eq!(greatest_common_divisor(&[u64::MAX]), u64::MAX);
        assert_eq!(greatest_common_divisor(&[u64::MAX, 0]), u64::MAX);
    }

    #[test]
    fn test_greatest_common_divisor_maximum_values() {
        assert_eq!(greatest_common_divisor(&[u64::MAX, u64::MAX - 1]), 1);
    }
}
