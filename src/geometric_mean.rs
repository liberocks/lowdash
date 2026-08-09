/// Returns the geometric mean of nonnegative finite values.
///
/// Empty input, negative values, and nonfinite values return `None`. Any zero
/// input makes the result `Some(0.0)`. Positive values are averaged in log
/// space to avoid intermediate product overflow.
///
/// **Time Complexity:** O(n) time and O(1) additional space.
///
/// # Examples
/// ```rust
/// use lowdash::geometric_mean;
///
/// assert_eq!(geometric_mean(&[1.0, 4.0, 16.0]), Some(4.0));
/// assert_eq!(geometric_mean(&[0.0, 2.0]), Some(0.0));
/// ```
pub fn geometric_mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let mut has_zero = false;
    let mut mean_log = 0.0;
    let mut count = 0.0;
    for &value in values {
        if !value.is_finite() || value < 0.0 {
            return None;
        }
        if value == 0.0 {
            has_zero = true;
        } else if !has_zero {
            count += 1.0;
            mean_log += (value.ln() - mean_log) / count;
        }
    }
    if has_zero {
        return Some(0.0);
    }

    Some(mean_log.exp())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_known_geometric_means() {
        assert_eq!(geometric_mean(&[1.0, 4.0, 16.0]), Some(4.0));
    }

    #[test]
    fn handles_empty_singleton_and_duplicates() {
        assert_eq!(geometric_mean(&[]), None);
        assert!((geometric_mean(&[9.0]).unwrap() - 9.0).abs() < 1e-12);
        assert!((geometric_mean(&[2.0, 2.0, 8.0]).unwrap() - 3.1748021039363987).abs() < 1e-12);
    }

    #[test]
    fn returns_zero_when_any_value_is_zero() {
        assert_eq!(geometric_mean(&[10.0, 0.0, 20.0]), Some(0.0));
    }

    #[test]
    fn rejects_negative_and_nonfinite_values() {
        assert_eq!(geometric_mean(&[-1.0, 1.0]), None);
        assert_eq!(geometric_mean(&[f64::NAN]), None);
        assert_eq!(geometric_mean(&[f64::INFINITY]), None);
    }

    #[test]
    fn avoids_product_overflow() {
        let result = geometric_mean(&[1.0e200, 1.0e200]).unwrap();

        assert!((result - 1.0e200).abs() / 1.0e200 < 1e-12);
    }
}
