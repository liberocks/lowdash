/// Returns the Pearson correlation coefficient for paired observations.
///
/// Inputs must have equal lengths, at least two pairs, finite values, and
/// nonzero variation in both dimensions. The result is computed with stable
/// one-pass centered sums and is clamped to the mathematical `[-1, 1]` range.
/// Invalid or constant input returns `None`.
///
/// **Time Complexity:** O(n) time and O(1) additional space.
///
/// # Examples
/// ```rust
/// use lowdash::correlation;
///
/// let result = correlation(&[1.0, 2.0, 3.0], &[2.0, 4.0, 6.0]).unwrap();
/// assert!((result - 1.0).abs() < 1e-12);
/// ```
pub fn correlation(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() < 2 || x.len() != y.len() {
        return None;
    }
    let mut count = 0.0;
    let mut mean_x = 0.0;
    let mut mean_y = 0.0;
    let mut sum_xx = 0.0;
    let mut sum_yy = 0.0;
    let mut sum_xy = 0.0;

    for (&value_x, &value_y) in x.iter().zip(y) {
        if !value_x.is_finite() || !value_y.is_finite() {
            return None;
        }
        count += 1.0;
        let difference_x = value_x - mean_x;
        let difference_y = value_y - mean_y;
        mean_x += difference_x / count;
        mean_y += difference_y / count;
        let updated_difference_x = value_x - mean_x;
        let updated_difference_y = value_y - mean_y;
        sum_xx += difference_x * updated_difference_x;
        sum_yy += difference_y * updated_difference_y;
        sum_xy += difference_x * updated_difference_y;
    }

    if !sum_xx.is_finite()
        || !sum_yy.is_finite()
        || !sum_xy.is_finite()
        || sum_xx <= 0.0
        || sum_yy <= 0.0
    {
        return None;
    }

    let denominator = sum_xx.sqrt() * sum_yy.sqrt();
    let result = sum_xy / denominator;
    result.is_finite().then_some(result.clamp(-1.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_perfect_positive_and_negative_correlations() {
        assert!((correlation(&[1.0, 2.0, 3.0], &[2.0, 4.0, 6.0]).unwrap() - 1.0).abs() < 1e-12);
        assert!((correlation(&[1.0, 2.0, 3.0], &[6.0, 4.0, 2.0]).unwrap() + 1.0).abs() < 1e-12);
    }

    #[test]
    fn calculates_a_known_partial_correlation() {
        let result = correlation(&[1.0, 2.0, 3.0], &[1.0, 3.0, 2.0]).unwrap();

        assert!((result - 0.5).abs() < 1e-12);
    }

    #[test]
    fn rejects_empty_singleton_and_mismatched_inputs() {
        assert_eq!(correlation(&[], &[]), None);
        assert_eq!(correlation(&[1.0], &[2.0]), None);
        assert_eq!(correlation(&[1.0, 2.0], &[2.0]), None);
    }

    #[test]
    fn rejects_constant_and_nonfinite_inputs() {
        assert_eq!(correlation(&[1.0, 1.0, 1.0], &[2.0, 3.0, 4.0]), None);
        assert_eq!(correlation(&[1.0, 2.0, 3.0], &[4.0, 4.0, 4.0]), None);
        assert_eq!(correlation(&[1.0, f64::NAN], &[2.0, 3.0]), None);
    }

    #[test]
    fn is_stable_for_large_offsets() {
        let x = [1_000_000_001.0, 1_000_000_002.0, 1_000_000_003.0];
        let y = [2_000_000_002.0, 2_000_000_004.0, 2_000_000_006.0];

        assert!((correlation(&x, &y).unwrap() - 1.0).abs() < 1e-12);
    }
}
