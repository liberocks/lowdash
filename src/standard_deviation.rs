/// Returns the sample standard deviation of `values`.
///
/// This is the square root of [`crate::sample_variance`]. Empty and singleton
/// inputs, nonfinite values, and nonfinite results return `None`.
///
/// **Time Complexity:** O(n) time and O(1) additional space.
///
/// # Examples
/// ```rust
/// use lowdash::standard_deviation;
///
/// let result = standard_deviation(&[1.0, 2.0, 3.0]);
/// assert!((result.unwrap() - 1.0).abs() < 1e-12);
/// ```
pub fn standard_deviation(values: &[f64]) -> Option<f64> {
    crate::sample_variance::sample_variance(values).map(f64::sqrt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_known_sample_standard_deviation() {
        let result = standard_deviation(&[1.0, 2.0, 3.0]);

        assert!((result.unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn rejects_empty_and_singleton_inputs() {
        assert_eq!(standard_deviation(&[]), None);
        assert_eq!(standard_deviation(&[42.0]), None);
    }

    #[test]
    fn handles_negative_values_and_duplicates() {
        assert!(
            (standard_deviation(&[-2.0, -1.0, 0.0, 1.0, 2.0]).unwrap() - 1.5811388300841898).abs()
                < 1e-12
        );
        assert_eq!(standard_deviation(&[3.0, 3.0, 3.0]), Some(0.0));
    }

    #[test]
    fn rejects_nonfinite_values() {
        assert_eq!(standard_deviation(&[1.0, f64::NAN]), None);
        assert_eq!(standard_deviation(&[1.0, f64::INFINITY]), None);
    }

    #[test]
    fn matches_the_square_root_of_sample_variance() {
        let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

        assert!((standard_deviation(&values).unwrap() - (32.0_f64 / 7.0).sqrt()).abs() < 1e-12);
    }
}
