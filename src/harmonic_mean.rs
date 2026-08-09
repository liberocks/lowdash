/// Returns the harmonic mean of nonnegative finite values.
///
/// Empty input, negative values, and nonfinite values return `None`. Any zero
/// input makes the result `Some(0.0)`. Positive inputs use a scaled reciprocal
/// mean to avoid overflowing individual reciprocals.
///
/// **Time Complexity:** O(n) time and O(1) additional space.
///
/// # Examples
/// ```rust
/// use lowdash::harmonic_mean;
///
/// assert_eq!(harmonic_mean(&[1.0, 3.0]), Some(1.5));
/// assert_eq!(harmonic_mean(&[0.0, 2.0]), Some(0.0));
/// ```
pub fn harmonic_mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    let mut minimum = f64::INFINITY;
    let mut reciprocal_sum = 0.0;
    let mut compensation = 0.0;
    for &value in values {
        if !value.is_finite() || value < 0.0 {
            return None;
        }
        if value == 0.0 {
            return Some(0.0);
        }

        let reciprocal = if minimum.is_infinite() {
            minimum = value;
            1.0
        } else if value < minimum {
            let scale = value / minimum;
            reciprocal_sum *= scale;
            compensation *= scale;
            minimum = value;
            1.0
        } else {
            minimum / value
        };
        let next = reciprocal_sum + reciprocal;
        if reciprocal_sum.abs() >= reciprocal.abs() {
            compensation += (reciprocal_sum - next) + reciprocal;
        } else {
            compensation += (reciprocal - next) + reciprocal_sum;
        }
        reciprocal_sum = next;
    }

    let mean_reciprocal = (reciprocal_sum + compensation) / values.len() as f64;
    let result = minimum / mean_reciprocal;
    result.is_finite().then_some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_known_harmonic_means() {
        assert_eq!(harmonic_mean(&[1.0, 3.0]), Some(1.5));
    }

    #[test]
    fn handles_empty_singleton_and_duplicates() {
        assert_eq!(harmonic_mean(&[]), None);
        assert_eq!(harmonic_mean(&[4.0]), Some(4.0));
        assert!((harmonic_mean(&[2.0, 2.0, 8.0]).unwrap() - 8.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn returns_zero_when_any_value_is_zero() {
        assert_eq!(harmonic_mean(&[10.0, 0.0, 20.0]), Some(0.0));
    }

    #[test]
    fn rejects_negative_and_nonfinite_values() {
        assert_eq!(harmonic_mean(&[-1.0, 1.0]), None);
        assert_eq!(harmonic_mean(&[f64::NAN]), None);
        assert_eq!(harmonic_mean(&[f64::INFINITY]), None);
    }

    #[test]
    fn handles_small_positive_values_without_reciprocal_overflow() {
        let result = harmonic_mean(&[1.0e-308, 1.0e-308]).unwrap();

        assert!((result - 1.0e-308).abs() < f64::EPSILON);
    }
}
