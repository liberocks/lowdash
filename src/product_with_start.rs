use std::ops::Mul;

/// Returns `start` multiplied by every value in `values`.
///
/// Unlike [`product`](crate::product), this function uses the caller-provided
/// starting value and returns it unchanged for empty input.
///
/// # Complexity
///
/// Runs in `O(n)` time and `O(1)` additional space.
///
/// # Examples
///
/// ```rust
/// use lowdash::product_with_start;
///
/// assert_eq!(product_with_start(&[2, 3, 4], 10), 240);
/// assert_eq!(product_with_start::<i32>(&[], 10), 10);
/// ```
pub fn product_with_start<T>(values: &[T], start: T) -> T
where
    T: Copy + Mul<Output = T>,
{
    values
        .iter()
        .copied()
        .fold(start, |result, value| result * value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_with_start_integers() {
        assert_eq!(product_with_start(&[2, 3, 4], 10), 240);
    }

    #[test]
    fn test_product_with_start_empty_returns_start() {
        assert_eq!(product_with_start::<i32>(&[], 10), 10);
        assert_eq!(product_with_start::<f64>(&[], 2.5), 2.5);
    }

    #[test]
    fn test_product_with_start_floats() {
        assert_eq!(product_with_start(&[1.5, 2.0, 3.0], 2.0), 18.0);
    }

    #[test]
    fn test_product_with_start_zero_and_single_value() {
        assert_eq!(product_with_start(&[0, 2, 3], 10), 0);
        assert_eq!(product_with_start(&[7], 3), 21);
    }
}
