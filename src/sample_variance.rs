/// Returns the sample variance using stable one-pass accumulation.
///
/// The divisor is `n - 1` (Bessel's correction). Empty and singleton inputs,
/// nonfinite values, and nonfinite results return `None`.
///
/// **Time Complexity:** O(n) time and O(1) additional space.
///
/// # Examples
/// ```rust
/// use lowdash::sample_variance;
///
/// assert_eq!(sample_variance(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(2.5));
/// assert_eq!(sample_variance(&[1.0]), None);
/// ```
pub fn sample_variance(values: &[f64]) -> Option<f64> {
    if values.len() < 2 {
        return None;
    }

    let mut count = 0.0;
    let mut mean = 0.0;
    let mut sum_squared_differences = 0.0;
    for &value in values {
        if !value.is_finite() {
            return None;
        }
        count += 1.0;
        let difference_from_mean = value - mean;
        mean += difference_from_mean / count;
        let updated_difference = value - mean;
        sum_squared_differences += difference_from_mean * updated_difference;
    }

    let result = sum_squared_differences / (count - 1.0);
    result.is_finite().then_some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_known_sample_variance() {
        assert_eq!(sample_variance(&[1.0, 2.0, 3.0, 4.0, 5.0]), Some(2.5));
    }

    #[test]
    fn rejects_empty_and_singleton_inputs() {
        assert_eq!(sample_variance(&[]), None);
        assert_eq!(sample_variance(&[42.0]), None);
    }

    #[test]
    fn handles_negative_values_and_duplicates() {
        assert!((sample_variance(&[-2.0, -1.0, 0.0, 1.0, 2.0]).unwrap() - 2.5).abs() < 1e-12);
        assert_eq!(sample_variance(&[3.0, 3.0, 3.0]), Some(0.0));
    }

    #[test]
    fn rejects_nonfinite_values() {
        assert_eq!(sample_variance(&[1.0, f64::NAN]), None);
        assert_eq!(sample_variance(&[1.0, f64::INFINITY]), None);
    }

    #[test]
    fn is_stable_for_large_offsets() {
        let values = [1_000_000_001.0, 1_000_000_002.0, 1_000_000_003.0];

        assert!((sample_variance(&values).unwrap() - 1.0).abs() < 1e-12);
    }
}
