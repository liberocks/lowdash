/// Returns the weighted arithmetic mean of `values`.
///
/// Inputs must be non-empty and have equal lengths. Every value and weight
/// must be finite, weights must be nonnegative, and the total weight must be
/// positive. Invalid input returns `None`.
///
/// **Time Complexity:** O(n) time and O(1) additional space.
///
/// # Examples
/// ```rust
/// use lowdash::weighted_mean;
///
/// assert_eq!(weighted_mean(&[1.0, 3.0], &[1.0, 3.0]), Some(2.5));
/// assert_eq!(weighted_mean(&[1.0], &[0.0]), None);
/// ```
pub fn weighted_mean(values: &[f64], weights: &[f64]) -> Option<f64> {
    if values.is_empty() || values.len() != weights.len() {
        return None;
    }

    let mut weighted_sum = 0.0;
    let mut weighted_compensation = 0.0;
    let mut total_weight = 0.0;
    let mut weight_compensation = 0.0;

    for (&value, &weight) in values.iter().zip(weights) {
        if !value.is_finite() || !weight.is_finite() || weight < 0.0 {
            return None;
        }

        let product = value * weight;
        if !product.is_finite() {
            return None;
        }

        let next_weighted_sum = weighted_sum + product;
        if weighted_sum.abs() >= product.abs() {
            weighted_compensation += (weighted_sum - next_weighted_sum) + product;
        } else {
            weighted_compensation += (product - next_weighted_sum) + weighted_sum;
        }
        weighted_sum = next_weighted_sum;

        let next_weight = total_weight + weight;
        if total_weight.abs() >= weight.abs() {
            weight_compensation += (total_weight - next_weight) + weight;
        } else {
            weight_compensation += (weight - next_weight) + total_weight;
        }
        total_weight = next_weight;
    }

    let total_weight = total_weight + weight_compensation;
    if total_weight <= 0.0 || !total_weight.is_finite() {
        return None;
    }

    let result = (weighted_sum + weighted_compensation) / total_weight;
    result.is_finite().then_some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_known_weighted_means() {
        assert_eq!(weighted_mean(&[1.0, 3.0], &[1.0, 3.0]), Some(2.5));
    }

    #[test]
    fn handles_empty_and_mismatched_inputs() {
        assert_eq!(weighted_mean(&[], &[]), None);
        assert_eq!(weighted_mean(&[1.0], &[]), None);
        assert_eq!(weighted_mean(&[1.0, 2.0], &[1.0]), None);
    }

    #[test]
    fn accepts_negative_values_and_zero_weights() {
        assert_eq!(weighted_mean(&[-2.0, 4.0], &[1.0, 1.0]), Some(1.0));
        assert_eq!(weighted_mean(&[10.0, 20.0], &[0.0, 1.0]), Some(20.0));
    }

    #[test]
    fn rejects_invalid_weights_and_nonpositive_totals() {
        assert_eq!(weighted_mean(&[1.0], &[-1.0]), None);
        assert_eq!(weighted_mean(&[1.0], &[f64::NAN]), None);
        assert_eq!(weighted_mean(&[1.0], &[f64::INFINITY]), None);
        assert_eq!(weighted_mean(&[1.0, 2.0], &[0.0, 0.0]), None);
    }

    #[test]
    fn rejects_nonfinite_values() {
        assert_eq!(weighted_mean(&[f64::NAN], &[1.0]), None);
        assert_eq!(weighted_mean(&[f64::INFINITY], &[1.0]), None);
    }

    #[test]
    fn handles_duplicate_values() {
        let result = weighted_mean(&[2.0, 2.0, 6.0], &[1.0, 1.0, 2.0]).unwrap();

        assert!((result - 4.0).abs() < 1e-12);
    }
}
