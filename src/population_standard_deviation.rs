/// Returns the population standard deviation of `values`.
///
/// This is the square root of the existing population [`crate::variance`].
/// Empty input, nonfinite input, and nonfinite results return `None`; a
/// singleton therefore returns `Some(0.0)`.
///
/// **Time Complexity:** O(n) time and O(1) additional space.
///
/// # Examples
/// ```rust
/// use lowdash::population_standard_deviation;
///
/// let result = population_standard_deviation(&[1.0, 2.0, 3.0]);
/// assert!((result.unwrap() - (2.0_f64 / 3.0).sqrt()).abs() < 1e-12);
/// ```
pub fn population_standard_deviation(values: &[f64]) -> Option<f64> {
    if values.iter().any(|value| !value.is_finite()) {
        return None;
    }

    crate::variance::variance(values).and_then(|variance| {
        let result = variance.sqrt();
        result.is_finite().then_some(result)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_known_population_standard_deviation() {
        let result = population_standard_deviation(&[1.0, 2.0, 3.0]);

        assert!((result.unwrap() - (2.0_f64 / 3.0).sqrt()).abs() < 1e-12);
    }

    #[test]
    fn handles_empty_singleton_and_duplicates() {
        assert_eq!(population_standard_deviation(&[]), None);
        assert_eq!(population_standard_deviation(&[42.0]), Some(0.0));
        assert_eq!(population_standard_deviation(&[3.0, 3.0, 3.0]), Some(0.0));
    }

    #[test]
    fn handles_negative_values() {
        let result = population_standard_deviation(&[-2.0, -1.0, 0.0, 1.0, 2.0]).unwrap();

        assert!((result - 1.4142135623730951).abs() < 1e-12);
    }

    #[test]
    fn rejects_nonfinite_values() {
        assert_eq!(population_standard_deviation(&[1.0, f64::NAN]), None);
        assert_eq!(population_standard_deviation(&[1.0, f64::INFINITY]), None);
    }
}
