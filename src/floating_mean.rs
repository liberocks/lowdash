/// Returns the arithmetic mean with compensated floating-point summation.
///
/// Empty input returns `None`. A `NaN` input is preserved as `Some(NaN)`;
/// infinities follow ordinary IEEE-754 arithmetic and may produce `NaN`.
///
/// **Time Complexity:** O(n) time and O(1) additional space.
///
/// # Examples
/// ```rust
/// use lowdash::floating_mean;
///
/// assert_eq!(floating_mean(&[1.0, 2.0, 3.0]), Some(2.0));
/// assert_eq!(floating_mean(&[]), None);
/// ```
pub fn floating_mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let mut sum = 0.0;
    let mut compensation = 0.0;
    for &value in values {
        let next = sum + value;
        if sum.abs() >= value.abs() {
            compensation += (sum - next) + value;
        } else {
            compensation += (value - next) + sum;
        }
        sum = next;
    }

    Some((sum + compensation) / values.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_known_means() {
        assert_eq!(floating_mean(&[1.0, 2.0, 3.0, 4.0]), Some(2.5));
    }

    #[test]
    fn handles_empty_singleton_and_negative_values() {
        assert_eq!(floating_mean(&[]), None);
        assert_eq!(floating_mean(&[42.5]), Some(42.5));
        assert_eq!(floating_mean(&[-1.0, -2.0, -3.0]), Some(-2.0));
    }

    #[test]
    fn preserves_duplicates() {
        assert_eq!(floating_mean(&[2.0, 2.0, 4.0, 4.0]), Some(3.0));
    }

    #[test]
    fn compensates_for_large_cancellation() {
        let result = floating_mean(&[1.0e16, 1.0, -1.0e16]).unwrap();

        assert!((result - (1.0 / 3.0)).abs() < 1e-15);
    }

    #[test]
    fn preserves_nan_behavior() {
        assert!(floating_mean(&[1.0, f64::NAN]).unwrap().is_nan());
    }
}
