/// Returns ordinary least-squares `(slope, intercept)` for paired data.
///
/// Inputs must have equal lengths, at least two finite pairs, and nonconstant
/// `x` values. The fit uses stable one-pass centered sums. Invalid input or a
/// nonfinite fit returns `None`.
///
/// **Time Complexity:** O(n) time and O(1) additional space.
///
/// # Examples
/// ```rust
/// use lowdash::linear_regression;
///
/// let (slope, intercept) = linear_regression(&[1.0, 2.0, 3.0], &[3.0, 5.0, 7.0]).unwrap();
/// assert!((slope - 2.0).abs() < 1e-12);
/// assert!((intercept - 1.0).abs() < 1e-12);
/// ```
pub fn linear_regression(x: &[f64], y: &[f64]) -> Option<(f64, f64)> {
    if x.len() < 2 || x.len() != y.len() {
        return None;
    }
    if x.iter().chain(y).any(|value| !value.is_finite()) {
        return None;
    }

    let mut count = 0.0;
    let mut mean_x = 0.0;
    let mut mean_y = 0.0;
    let mut sum_xx = 0.0;
    let mut sum_xy = 0.0;

    for (&value_x, &value_y) in x.iter().zip(y) {
        count += 1.0;
        let difference_x = value_x - mean_x;
        let difference_y = value_y - mean_y;
        mean_x += difference_x / count;
        mean_y += difference_y / count;
        sum_xx += difference_x * (value_x - mean_x);
        sum_xy += difference_x * (value_y - mean_y);
    }

    if !sum_xx.is_finite() || !sum_xy.is_finite() || sum_xx <= 0.0 {
        return None;
    }

    let slope = sum_xy / sum_xx;
    let intercept = mean_y - slope * mean_x;
    (slope.is_finite() && intercept.is_finite()).then_some((slope, intercept))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_a_known_positive_regression() {
        let (slope, intercept) = linear_regression(&[1.0, 2.0, 3.0], &[3.0, 5.0, 7.0]).unwrap();

        assert!((slope - 2.0).abs() < 1e-12);
        assert!((intercept - 1.0).abs() < 1e-12);
    }

    #[test]
    fn calculates_negative_and_duplicate_x_values() {
        let (slope, intercept) =
            linear_regression(&[-1.0, 0.0, 0.0, 1.0], &[3.0, 1.0, 1.0, -1.0]).unwrap();

        assert!((slope + 2.0).abs() < 1e-12);
        assert!((intercept - 1.0).abs() < 1e-12);
    }

    #[test]
    fn rejects_empty_singleton_and_mismatched_inputs() {
        assert_eq!(linear_regression(&[], &[]), None);
        assert_eq!(linear_regression(&[1.0], &[2.0]), None);
        assert_eq!(linear_regression(&[1.0, 2.0], &[3.0]), None);
    }

    #[test]
    fn rejects_constant_x_and_nonfinite_values() {
        assert_eq!(linear_regression(&[1.0, 1.0, 1.0], &[2.0, 3.0, 4.0]), None);
        assert_eq!(linear_regression(&[1.0, f64::NAN], &[2.0, 3.0]), None);
        assert_eq!(linear_regression(&[1.0, 2.0], &[2.0, f64::INFINITY]), None);
    }

    #[test]
    fn is_stable_for_large_offsets() {
        let x = [1_000_000_001.0, 1_000_000_002.0, 1_000_000_003.0];
        let y = [2_000_000_003.0, 2_000_000_005.0, 2_000_000_007.0];
        let (slope, intercept) = linear_regression(&x, &y).unwrap();

        assert!((slope - 2.0).abs() < 1e-12);
        assert!((intercept - 1.0).abs() < 1e-6);
    }
}
