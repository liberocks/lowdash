/// Returns the sample covariance of paired observations.
///
/// Inputs must have equal lengths and contain at least two finite values. The
/// calculation uses a stable one-pass update and divides by `n - 1`.
///
/// **Time Complexity:** O(n) time and O(1) additional space.
///
/// # Examples
/// ```rust
/// use lowdash::covariance;
///
/// assert_eq!(covariance(&[1.0, 2.0, 3.0], &[2.0, 4.0, 6.0]), Some(2.0));
/// ```
pub fn covariance(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() < 2 || x.len() != y.len() {
        return None;
    }
    let mut count = 0.0;
    let mut mean_x = 0.0;
    let mut mean_y = 0.0;
    let mut co_moment = 0.0;

    for (&value_x, &value_y) in x.iter().zip(y) {
        if !value_x.is_finite() || !value_y.is_finite() {
            return None;
        }
        count += 1.0;
        let difference_x = value_x - mean_x;
        mean_x += difference_x / count;
        mean_y += (value_y - mean_y) / count;
        co_moment += difference_x * (value_y - mean_y);
    }

    let result = co_moment / (count - 1.0);
    result.is_finite().then_some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_known_positive_and_negative_covariance() {
        assert_eq!(covariance(&[1.0, 2.0, 3.0], &[2.0, 4.0, 6.0]), Some(2.0));
        assert_eq!(covariance(&[1.0, 2.0, 3.0], &[3.0, 2.0, 1.0]), Some(-1.0));
    }

    #[test]
    fn rejects_empty_singleton_and_mismatched_inputs() {
        assert_eq!(covariance(&[], &[]), None);
        assert_eq!(covariance(&[1.0], &[2.0]), None);
        assert_eq!(covariance(&[1.0, 2.0], &[2.0]), None);
    }

    #[test]
    fn handles_duplicates_and_constant_pairs() {
        assert_eq!(covariance(&[1.0, 1.0, 1.0], &[2.0, 4.0, 6.0]), Some(0.0));
        assert_eq!(covariance(&[1.0, 2.0, 1.0], &[3.0, 3.0, 3.0]), Some(0.0));
    }

    #[test]
    fn rejects_nonfinite_values() {
        assert_eq!(covariance(&[1.0, f64::NAN], &[2.0, 3.0]), None);
        assert_eq!(covariance(&[1.0, 2.0], &[2.0, f64::INFINITY]), None);
    }

    #[test]
    fn is_stable_for_large_offsets() {
        let x = [1_000_000_001.0, 1_000_000_002.0, 1_000_000_003.0];
        let y = [2_000_000_002.0, 2_000_000_004.0, 2_000_000_006.0];

        assert!((covariance(&x, &y).unwrap() - 2.0).abs() < 1e-12);
    }
}
