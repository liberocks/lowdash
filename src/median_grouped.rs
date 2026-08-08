/// Estimates a grouped-data median from sorted class-midpoint observations.
///
/// `interval` is the fixed, positive class width. The upper middle midpoint
/// is used for even-length input. If `m` is that midpoint, `L = m - interval / 2`,
/// `cf` is the count below `m`, and `f` is the count equal to `m`, the estimate
/// is `L + interval * (n / 2 - cf) / f`. Empty input, nonfinite values, or an
/// interval that is not finite and positive returns `None`.
///
/// **Time Complexity:** O(n log n) time and O(n) space.
///
/// # Examples
/// ```rust
/// use lowdash::median_grouped;
///
/// let values = [10.0, 10.0, 20.0, 20.0, 20.0, 30.0];
/// let result = median_grouped(&values, 10.0).unwrap();
/// assert!((result - 18.333333333333336).abs() < 1e-12);
/// ```
pub fn median_grouped(values: &[f64], interval: f64) -> Option<f64> {
    if values.is_empty() || !interval.is_finite() || interval <= 0.0 {
        return None;
    }
    if values.iter().any(|value| !value.is_finite()) {
        return None;
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(|left, right| left.partial_cmp(right).unwrap());
    let midpoint = sorted[sorted.len() / 2];
    let cumulative = sorted.iter().take_while(|value| **value < midpoint).count() as f64;
    let frequency = sorted.iter().filter(|value| **value == midpoint).count() as f64;
    let lower_boundary = midpoint - interval / 2.0;
    let result = lower_boundary + interval * (sorted.len() as f64 / 2.0 - cumulative) / frequency;

    result.is_finite().then_some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimates_a_known_grouped_median() {
        let values = [10.0, 10.0, 20.0, 20.0, 20.0, 30.0];

        assert!((median_grouped(&values, 10.0).unwrap() - 18.333333333333336).abs() < 1e-12);
    }

    #[test]
    fn handles_empty_singleton_and_duplicates() {
        assert_eq!(median_grouped(&[], 1.0), None);
        assert_eq!(median_grouped(&[5.0], 2.0), Some(5.0));
        assert_eq!(median_grouped(&[2.0, 2.0, 2.0], 4.0), Some(2.0));
    }

    #[test]
    fn handles_unsorted_and_negative_midpoints() {
        let values = [0.0, -10.0, 10.0, 0.0, -10.0, 0.0];
        let result = median_grouped(&values, 10.0).unwrap();

        assert!((result - (-5.0 / 3.0)).abs() < 1e-12);
    }

    #[test]
    fn rejects_invalid_intervals() {
        let values = [1.0, 2.0, 3.0];

        assert_eq!(median_grouped(&values, 0.0), None);
        assert_eq!(median_grouped(&values, -1.0), None);
        assert_eq!(median_grouped(&values, f64::NAN), None);
        assert_eq!(median_grouped(&values, f64::INFINITY), None);
    }

    #[test]
    fn rejects_nonfinite_values() {
        assert_eq!(median_grouped(&[1.0, f64::NAN], 1.0), None);
        assert_eq!(median_grouped(&[f64::INFINITY], 1.0), None);
    }
}
